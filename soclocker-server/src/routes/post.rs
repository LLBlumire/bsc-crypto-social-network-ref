use crate::{
    routes::auth::auth_internal,
    database::CoreDbConn,
    models::{NoaInsert, Post, PostData, PostInsert, PostResponse, User, PostPutData},
    schema::{
        Posts::{
            columns::{
                Content as PostContent,
                Nonce as PostNonce,
                TimePosted,
                UserID as PostUserID,
                ID as PostID,
            },
            table as Posts,
        },
        Users::{
            columns::{PublicKey, Username, ID as UserID},
            table as Users,
        },
        NOA::table as NOA,
    },
};
use chrono::Utc;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::{get, http::Status, post, put};
use rocket_contrib::json::Json;

#[post("/post", data = "<post_data>")]
pub fn post(conn: CoreDbConn, post_data: Json<PostData>) -> Result<Json<bool>, Status> {
    let post_data = post_data.into_inner();

    let post_creator_id = Users
        .filter(Username.eq(&post_data.username))
        .select(UserID)
        .first::<i32>(&conn.0)
        .map_err(|_| Status::InternalServerError)?;

    let now = diesel::dsl::now;

    if !auth_internal(&conn, &post_data.proof, &post_data.username) {
        return Err(Status::Forbidden);
    }

    let inserted_post_id = diesel::insert_into(Posts)
        .values(&PostInsert {
            content: &post_data.content,
            nonce: &post_data.nonce,
            user_id: post_creator_id,
            time_posted: now,
            public_key: &post_data.public_key,
            public_key_nonce: &post_data.public_key_nonce
        })
        .execute(&conn.0)
        .and_then(|_| {
            Posts
                .filter(PostNonce.eq(&post_data.nonce))
                .filter(PostUserID.eq(post_creator_id))
                .select(PostID)
                .first::<i32>(&conn.0)
        })
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(post_data.noa_encrypted_keys.into_iter().all(|noa| {
        Users
            .filter(Username.eq(&noa.username))
            .first::<User>(&conn.0)
            .and_then(|noa_user| {
                diesel::insert_into(NOA)
                    .values(&NoaInsert {
                        user_id: noa_user.id,
                        post_id: inserted_post_id,
                        secret_key: &noa.encrypted_secret_key,
                        nonce: &noa.nonce,
                    })
                    .execute(&conn.0)
            })
            .is_ok()
    })))
}

#[put("/post", data = "<put_data>")]
pub fn put(conn: CoreDbConn, put_data: Json<PostPutData>) -> Result<Status, Status> {
    let put_data = put_data.into_inner();
    let (username, post_id) = Posts
        .inner_join(Users.on(UserID.eq(PostUserID)))
        .filter(PostID.eq(&put_data.post_id))
        .select((Username, PostID))
        .first::<(String, i32)>(&conn.0)
        .map_err(|_| Status::NotFound)?;

    if !auth_internal(&conn, &put_data.proof, &username) {
        return Err(Status::Forbidden);
    }

    return Ok(diesel::update(Posts.filter(PostID.eq(post_id)))
        .set((PostContent.eq(&put_data.new_content), PostNonce.eq(&put_data.new_nonce)))
        .execute(&conn.0)
        .map(|_| Status::Ok)
        .map_err(|_| Status::InternalServerError)?);
}
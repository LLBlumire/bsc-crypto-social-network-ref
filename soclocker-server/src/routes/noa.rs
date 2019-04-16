use crate::{
    database::CoreDbConn,
    models::{NoaOuterResponse, NoaResponse, PostResponse},
    schema::{
        Posts::{
            columns::{
                Content as PostContent,
                Nonce as PostNonce,
                TimePosted,
                UserID as PostUserID,
                ID as PostID,
                PublicKey as EncryptedPublicKey,
                PublicKeyNonce as EncryptedPublicKeyNonce
            },
            table as Posts,
        },
        Users::{
            columns::{PublicKey as UserPublicKey, Username, ID as UserID},
            table as Users,
        },
        NOA::{
            columns::{
                Nonce as SecretKeyNonce,
                PostID as NOAPostID,
                SecretKey,
                UserID as NOAUserID,
            },
            table as NOA,
        },
    },
};
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::get;
use rocket_contrib::json::Json;

#[get("/noa?<username>&<skip>")]
pub fn get(conn: CoreDbConn, username: String, skip: Option<i64>) -> Json<NoaOuterResponse> {
    NOA.inner_join(Users.on(NOAUserID.eq(UserID)))
        .filter(Username.eq(&username))
        .select(diesel::dsl::count(NOAPostID))
        .first(&conn.0)
        .and_then(|d_count: i64| {
            Ok(
                Json(
                    NoaOuterResponse {
                        noas:
                            NOA.inner_join(Users.on(NOAUserID.eq(UserID)))
                                .inner_join(Posts.on(PostID.eq(NOAPostID)))
                                .filter(Username.eq(&username))
                                .limit(25)
                                .offset(skip.unwrap_or(0) * 25)
                                .order_by(TimePosted.desc())
                                .select((NOAPostID, SecretKey, SecretKeyNonce))
                                .load(&conn.0)
                                .unwrap_or(vec![])
                                .into_iter()
                                .filter_map(
                                    |(post_id, secret_key, secret_key_nonce): (
                                        i32,
                                        String,
                                        String,
                                    )|
                                     -> Option<NoaResponse> {
                                        Some(NoaResponse {
                                            post: Posts
                                                .inner_join(Users.on(UserID.eq(PostUserID)))
                                                .filter(PostID.eq(post_id))
                                                .select((
                                                    PostContent,
                                                    PostNonce,
                                                    Username,
                                                    UserPublicKey,
                                                    PostID,
                                                    TimePosted,
                                                    EncryptedPublicKey,
                                                    EncryptedPublicKeyNonce,
                                                ))
                                                .first::<PostResponse>(&conn.0)
                                                .ok()?,
                                            encrypted_secret_key: secret_key,
                                            nonce: secret_key_nonce,
                                            all_readers: NOA
                                                .inner_join(Users.on(UserID.eq(NOAUserID)))
                                                .filter(NOAPostID.eq(post_id))
                                                .select(Username)
                                                .load::<String>(&conn.0).ok()?
                                        })
                                    },
                                )
                                .collect(),
                        pages: if d_count % 25 == 0 { d_count / 25 } else { (d_count / 25) + 1 },
                    },
                ),
            )
        })
        .unwrap()
}

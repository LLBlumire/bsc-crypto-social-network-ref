use crate::{
    database::CoreDbConn,
    models::{User, UserInsert},
    schema::Users::{columns::Username, table as Users},
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{get, http::Status, post};
use rocket_contrib::json::Json;

#[get("/user?<username>")]
pub fn get(conn: CoreDbConn, username: String) -> Option<Json<User>> {
    Some(Json(Users.filter(Username.eq(&username)).first::<User>(&conn.0).ok()?))
}

#[post("/user", data = "<user_data>")]
pub fn post(conn: CoreDbConn, user_data: Json<UserInsert>) -> Result<Status, Status> {
    if Users.filter(Username.eq(&user_data.username)).first::<User>(&conn.0).is_ok() {
        return Err(Status::Conflict);
    }
    if let Ok(_) = diesel::insert_into(Users).values(&user_data.into_inner()).execute(&conn.0) {
        return Ok(Status::Ok);
    } else {
        return Err(Status::InternalServerError);
    }
}

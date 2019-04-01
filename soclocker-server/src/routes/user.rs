//! Contains the routing control for the `user` endpoint.

use crate::{
    database::CoreDbConn,
    models::{User, UserInsert},
    schema::Users::{columns::Username, table as Users},
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{get, http::Status, post};
use rocket_contrib::json::Json;

/// The `user` endpoint can be sent a GET request with a query string specifying
/// it's parameters in the format `?username=<USERNAME>`. This will return the
/// ID and PublicKey of the requested Username.
///
/// It responds with either `200 OK` and
///
/// ```json
/// {
///     id: 0,
///     publicKey: "...",
///     username: "..."
/// }
/// ```
///
/// or `404 Not Found` if the user does not exist.
#[get("/user?<username>")]
pub fn get(conn: CoreDbConn, username: String) -> Option<Json<User>> {
    Some(Json(Users.filter(Username.eq(&username)).first::<User>(&conn.0).ok()?))
}

/// The `user` endpoint can be sent a POST request with a body of
///
/// ```json
/// {
///     publicKey: "...",
///     username: "..."
/// }
/// ```
///
/// and it will attempt to insert them into the database. Responding
/// `201 Created` on success, `409 Conflict` if a user of that name already
/// exists, and `500 Internal Server Error` if there is a database error.
#[post("/user", data = "<user_data>")]
pub fn post(conn: CoreDbConn, user_data: Json<UserInsert>) -> Result<Status, Status> {
    if Users.filter(Username.eq(&user_data.username)).first::<User>(&conn.0).is_ok() {
        return Err(Status::Conflict);
    }
    if let Ok(_) = diesel::insert_into(Users).values(&user_data.into_inner()).execute(&conn.0) {
        return Ok(Status::Created);
    } else {
        return Err(Status::InternalServerError);
    }
}

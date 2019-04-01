use crate::{
    database::CoreDbConn,
    models::{AuthInsert, AuthResponse, AuthValidate},
    schema::{
        Auth::dsl::{PublicKey as AuthPublicKey, *},
        Users::dsl::{PublicKey as UsersPublicKey, *},
    },
    TIMEOUT_SECONDS,
};
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{dsl, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl, RunQueryDsl};
use rand::{rngs::OsRng, Rng};
use rocket::{get, http::Status, post, State};
use rocket_contrib::json::Json;
use sodiumoxide::crypto::box_ as pkc;

type Validator = [u8; 32];

#[post("/auth", data = "<verify>")]
pub fn post(conn: CoreDbConn, verify: Json<AuthValidate>) -> Json<bool> {
    Json(
        Users
            .inner_join(Auth.on(UsersPublicKey.eq(AuthPublicKey)))
            .select(dsl::count(Username))
            .filter(Username.eq(&verify.username))
            .filter(ExpectedToken.eq(&verify.decrypted_token))
            .first::<i64>(&conn.0)
            != Ok(0),
    )
}

#[get("/auth?<username>")]
pub fn get(
    conn: CoreDbConn,
    username: String,
    server_public: State<pkc::PublicKey>,
    server_secret: State<pkc::SecretKey>,
) -> Result<Json<AuthResponse>, Status> {
    let now = Utc::now().naive_utc();

    match Users
        .left_join(Auth.on(UsersPublicKey.eq(AuthPublicKey)))
        .select((UsersPublicKey, ExpectedToken.nullable(), Timeout.nullable()))
        .filter(Username.eq(&username))
        .first::<(String, Option<String>, Option<NaiveDateTime>)>(&conn.0)
    {
        Ok((public_key, Some(token), Some(timeout))) => {
            if timeout.timestamp() < now.timestamp() {
                diesel::delete(Auth.filter(AuthPublicKey.eq(public_key)))
                    .execute(&conn.0)
                    .map_err(|_| Status::InternalServerError)?;
                return get(conn, username, server_public, server_secret);
            }
            // Respond to user with token encrypted
            let public_key =
                pkc::PublicKey::from_slice(&base64::decode(&public_key).unwrap()).unwrap();
            let nonce = pkc::gen_nonce();
            let message =
                pkc::seal(&base64::decode(&token).unwrap(), &nonce, &public_key, &server_secret);
            return Ok(Json(AuthResponse {
                encrypted_token: base64::encode(&message),
                nonce: base64::encode(&nonce),
            }));
        },

        Ok((public_key, ..)) => {
            // Create a new token
            let validator: Validator = OsRng::new().expect("Could not Acquire OS Rng").gen();
            diesel::insert_into(Auth)
                .values(&AuthInsert {
                    public_key: &public_key,
                    expected_token: &base64::encode(&validator),
                    timeout: now + Duration::seconds(TIMEOUT_SECONDS),
                })
                .execute(&conn.0)
                .unwrap();

            return get(conn, username, server_public, server_secret);
        },

        Err(_) => {
            // Report an error
            return Err(Status::BadRequest);
        },
    }
}

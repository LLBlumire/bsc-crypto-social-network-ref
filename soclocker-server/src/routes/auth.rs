//! Contains the routing control for the `auth` endpoint.

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

/// Represents the size of data to use for randomly generating a validation key.
/// The larger the value, the harder the bruteforce.
type Validator = [u8; 32];

/// The `auth` endpoint can be sent a POST request with a body of
///
/// ```json
/// {
///     decryptedToken: "...",
///     username: "..."
/// }
/// ```
///
/// The response is `200 OK` with a body of either `true` if the validation is
/// successful, or `false` if the validation is unsuccessful.
///
/// This will usually be called after a GET request on the same endpoint, which
/// provides the encrypted data used for this verification as described beneath.
#[post("/auth", data = "<verify>")]
pub fn post(conn: CoreDbConn, verify: Json<AuthValidate>) -> Json<bool> {
    return Json(auth_internal(&conn, &verify.decrypted_token, &verify.username));
}

pub fn auth_internal(conn: &CoreDbConn, token: &str, username: &str) -> bool {
    let auth_valid: bool = Users
        .inner_join(Auth.on(UsersPublicKey.eq(AuthPublicKey)))
        .select(dsl::count(Username))
        .filter(Username.eq(username))
        .filter(ExpectedToken.eq(token))
        .first::<i64>(&conn.0)
        != Ok(0);

    if auth_valid {
        diesel::delete(Auth.filter(ExpectedToken.eq(token))).execute(&conn.0).unwrap();
    }

    return auth_valid;
}

/// The `auth` endpoint can be sent a GET request with a query string specifying
/// it's paramaters of `?username=<USERNAME>`. This will request a new
/// authentication process be established for the username supplied, and will
/// send the information necessary for this verification. Because the server
/// has no knowledge of private keys this is done by sending an encrypted
/// message to the client and expecting them to respond with its decrypted form.
///
/// If the username exists, the server will respond `200 OK` with a body of
///
/// ```json
/// {
///     encryptedToken: String,
///     nonce: String,
/// }
/// ```
///
/// If the username does not exist, the server will respond `404 Not Found`.
#[get("/auth?<username>")]
pub fn get(
    conn: CoreDbConn,
    username: String,
    server_secret: State<pkc::SecretKey>,
) -> Result<Json<AuthResponse>, Status> {
    let now = Utc::now().naive_utc();

    // Joins Auth with the users. This is done as a left join to enable
    // detection of any pre-existing authentication keys, and handle them as
    // appropriate.
    // ```sql
    // SELECT
    //  Users.PublicKey
    //  ExpectedToken
    //  Timeout
    // FROM Auth
    // LEFT JOIN Users ON Users.PublicKey = Auth.PublicKey
    // WHERE Username = <username>
    // LIMIT 1
    // ```
    match Users
        .left_join(Auth.on(UsersPublicKey.eq(AuthPublicKey)))
        .select((UsersPublicKey, ExpectedToken.nullable(), Timeout.nullable()))
        .filter(Username.eq(&username))
        .first::<(String, Option<String>, Option<NaiveDateTime>)>(&conn.0)
    {
        // In the instance where the User already has a token and timeout, and
        // thus has established a login protocol previously.
        Ok((public_key, Some(token), Some(timeout))) => {
            // If the timout has elapsed, and thus the authentication method is
            // no longer valid: delete the existing authentication method, and
            // trigger a new internal GET request on this endpoint to process
            // the users request again with their pre-existing authentication
            // attempt cleared.
            if timeout.timestamp() < now.timestamp() {
                // ```sql
                // DELETE FROM Auth
                // WHERE PublicKey = {public_key}
                // ```
                diesel::delete(Auth.filter(AuthPublicKey.eq(public_key)))
                    .execute(&conn.0)
                    .map_err(|_| Status::InternalServerError)?;
                return get(conn, username, server_secret);
            }
            // Otherwise, the user has a pre-existing authentication token and
            // it has not timed out, and thus can be re-used. Evaluate the users
            // public key, and encrypt the message to be used for verification
            // with their public key and the servers secret key.
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

        // If the user exists, but they do not have an existing authentication
        // method.
        Ok((public_key, ..)) => {
            // Create a new authentication token to be used for validation.
            let validator: Validator = OsRng::new().expect("Could not Acquire OS Rng").gen();
            // Store this authentication method, to be accessed by a future GET
            // request.
            // ```sql
            // INSERT INTO Auth
            // Values({public_key}, {validator}, {now + TIMEOUT_SECONDS})
            // ```
            diesel::insert_into(Auth)
                .values(&AuthInsert {
                    public_key: &public_key,
                    expected_token: &base64::encode(&validator),
                    timeout: now + Duration::seconds(TIMEOUT_SECONDS),
                })
                .execute(&conn.0)
                .unwrap();
            // Trigger a new internal GET request on this endpoint to process
            // the users request again with a valid and non timed out method of
            // authentication having been generated for them.
            return get(conn, username, server_secret);
        },

        // In the event the user does not exist, respond with a NotFound error.
        Err(_) => {
            return Err(Status::NotFound);
        },
    }
}

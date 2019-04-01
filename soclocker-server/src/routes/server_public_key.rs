//! Contains the routing control for the `server_public_key` endpoint.

use rocket::{get, State};
use rocket_contrib::json::Json;
use sodiumoxide::crypto::box_ as pkc;

/// The `server_public_key` endpoint can be sent a GET request which should
/// always respond `200 OK` with the body: `"..."` containing the servers public
/// key. This should never change once the server is deployed, as changes to it
/// will break all existing publically visible content and authentication
/// records.
#[get("/server_public_key")]
pub fn get(server_public: State<pkc::PublicKey>) -> Json<String> {
    Json(base64::encode(&server_public.0))
}
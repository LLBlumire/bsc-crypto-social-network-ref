use rocket::{get, State};
use rocket_contrib::json::Json;
use sodiumoxide::crypto::box_ as pkc;

#[get("/server_public_key")]
pub fn get(server_public: State<pkc::PublicKey>) -> Json<String> {
    Json(base64::encode(&server_public.0))
}

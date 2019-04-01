use crate::schema::{Auth, Users};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub public_key: String,
    pub username: String,
}

#[derive(Queryable, Serialize, Insertable, Debug, Deserialize, Default)]
#[table_name = "Users"]
pub struct UserInsert {
    #[column_name = "PublicKey"]
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[column_name = "Username"]
    pub username: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "Auth"]
pub struct AuthInsert<'a> {
    #[column_name = "PublicKey"]
    pub public_key: &'a str,
    #[column_name = "ExpectedToken"]
    pub expected_token: &'a str,
    #[column_name = "Timeout"]
    pub timeout: NaiveDateTime,
}

#[derive(Debug, Default, Serialize)]
pub struct AuthResponse {
    #[serde(rename = "encryptedToken")]
    pub encrypted_token: String,
    pub nonce: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct AuthValidate {
    #[serde(rename = "decryptedToken")]
    pub decrypted_token: String,
    pub username: String,
}

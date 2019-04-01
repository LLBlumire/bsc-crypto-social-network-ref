//! This module contains all the local structure definitions of models used by 
//! the database and API.

use crate::schema::{Auth, Users};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Represents an instance of a User as the exist in the database.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Default, Deserialize, Queryable, Serialize)]
pub struct User {
    /// A users unique identifier.
    pub id: i32,

    /// The public key a user has registered under.
    #[serde(rename = "publicKey")]
    pub public_key: String,

    /// The username a user has registered and aliased to their public key.
    pub username: String,
}

/// Used to receive posts to the `user` endpoint and insert them into the
/// database.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Default, Deserialize, Queryable, Serialize, Insertable)]
#[table_name = "Users"]
pub struct UserInsert {
    /// The public key the user wishes to register.
    #[column_name = "PublicKey"]
    #[serde(rename = "publicKey")]
    pub public_key: String,

    /// The username the user wishes to alias to their
    #[column_name = "Username"]
    pub username: String,
}

/// Used to insert Auth records to insert them into the database.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Queryable, Serialize, Insertable)]
#[table_name = "Auth"]
pub struct AuthInsert<'a> {
    /// The public key of the user being authenticated
    #[column_name = "PublicKey"]
    pub public_key: &'a str,
    
    /// The token expected to be provided by the client for valid authentication
    #[column_name = "ExpectedToken"]
    pub expected_token: &'a str,
    
    /// The time at which the authentication will become invalid, and a new
    /// means of authentication must be generated.
    #[column_name = "Timeout"]
    pub timeout: NaiveDateTime,
}

/// Response given to the user when they query for an authentication key
#[derive(Debug, PartialEq, Eq, Clone, Hash, Default, Deserialize, Queryable, Serialize)]
pub struct AuthResponse {
    /// The token a user must decrypt to authenticate themselves.
    #[serde(rename = "encryptedToken")]
    pub encrypted_token: String,

    /// The decryption nonce for the `encrypted_token`.
    pub nonce: String,
}

/// Format expected on authentication to validate the user and confirm their
/// identity
#[derive(Debug, PartialEq, Eq, Clone, Hash, Default, Deserialize, Queryable, Serialize)]
pub struct AuthValidate {
    /// The decrypted token the user is providing as proof of identity.
    #[serde(rename = "decryptedToken")]
    pub decrypted_token: String,

    /// The username the user is attempting to authenticate as.
    pub username: String,
}

//! This module contains all the local structure definitions of models used by
//! the database and API.

use crate::schema::{Auth, Posts, Users, NOA};
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
#[derive(
    Debug, PartialEq, Eq, Clone, Hash, Default, Deserialize, Queryable, Serialize, Insertable,
)]
#[table_name = "Users"]
pub struct UserInsert<'a, 'b> {
    /// The public key the user wishes to register.
    #[column_name = "PublicKey"]
    #[serde(rename = "publicKey")]
    pub public_key: &'a str,

    /// The username the user wishes to alias to their
    #[column_name = "Username"]
    pub username: &'b str,
}

/// Used to insert Auth records to insert them into the database.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Queryable, Serialize, Insertable)]
#[table_name = "Auth"]
pub struct AuthInsert<'a, 'b> {
    /// The public key of the user being authenticated
    #[column_name = "PublicKey"]
    pub public_key: &'a str,

    /// The token expected to be provided by the client for valid authentication
    #[column_name = "ExpectedToken"]
    pub expected_token: &'b str,

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
pub struct AuthValidate<'a> {
    /// The decrypted token the user is providing as proof of identity.
    #[serde(rename = "decryptedToken")]
    pub decrypted_token: &'a str,

    /// The username the user is attempting to authenticate as.
    pub username: String,
}

/// Used to insert new posts into the database.
#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "Posts"]
pub struct PostInsert<'a, 'b, 'c, 'd> {
    /// The encrypted conent of the post
    #[column_name = "Content"]
    pub content: &'a str,

    /// The nonce for decrypting the post
    #[column_name = "Nonce"]
    pub nonce: &'b str,

    /// The user ID
    #[column_name = "UserID"]
    pub user_id: i32,

    /// The time a post was created
    #[column_name = "TimePosted"]
    pub time_posted: diesel::dsl::now,

    /// The encrypted public key which encrypted the content
    #[column_name = "PublicKey"]
    pub public_key: &'c str,

    /// The nonce to decrypt the public key which encrypted the content
    #[column_name = "PublicKeyNonce"]
    pub public_key_nonce: &'d str,
}

/// Used to insert new NOAs into the database
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Queryable, Serialize, Insertable)]
#[table_name = "NOA"]
pub struct NoaInsert<'a, 'b> {
    /// The ID of the User being granted Access
    #[column_name = "UserID"]
    pub user_id: i32,

    /// The ID of the Post that Access is being granted on.
    #[column_name = "PostID"]
    pub post_id: i32,

    /// The encrypted private key
    #[column_name = "SecretKey"]
    pub secret_key: &'a str,

    /// The nonce for decrypting the private key
    #[column_name = "Nonce"]
    pub nonce: &'b str,
}

/// Represents the full database form of a post
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Queryable, Serialize)]
pub struct Post {
    /// The post ID
    pub id: i32,

    /// The post content
    pub content: String,

    /// The nonce the content is encrytped with
    pub nonce: String,

    /// The ID of the user who created the post
    #[serde(rename = "userID")]
    pub user_id: i32,

    /// The time the post was created
    #[serde(rename = "timePosted")]
    pub time_posted: NaiveDateTime,

    /// The encrypted public key
    #[serde(rename = "publicKey")]
    pub public_key: String,

    /// The nonce used to encrypt the public key
    #[serde(rename = "publicKeyNonce")]
    pub public_key_nonce: String
}

/// Used to insert new posts into the database
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Queryable, Serialize)]
pub struct PostData {
    /// The content of the post
    pub content: String,

    /// The nonce used to encrypt the post
    pub nonce: String,

    /// The username of the user who created the post
    pub username: String,

    /// The authentication token for proof of identity
    pub proof: String,

    /// The encrypted public key used to encode the content
    #[serde(rename = "publicKey")]
    pub public_key: String,

    /// The nonce used to encrypt the public key which encoded the content
    #[serde(rename="publicKeyNonce")]
    pub public_key_nonce: String,

    /// The user id and corresponding encrypted secret keys, as well as the
    /// nonce used for the encryption.
    #[serde(rename = "noaEncryptedKeys")]
    pub noa_encrypted_keys: Vec<PostNOATarget>,
}

/// Represents a single NOA target for use within PostData
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Queryable, Serialize)]
pub struct PostNOATarget {
    /// The user being granted access to the Post
    pub username: String,

    /// The encrypted secret key to access the Post
    #[serde(rename = "encryptedSecretKey")]
    pub encrypted_secret_key: String,

    /// The nonce used to decrypt the encrypted secret key.
    pub nonce: String,
}

/// Represents a response from the POST endpoint
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Queryable, Serialize)]
pub struct PostResponse {
    /// The encrypted psot body
    #[serde(rename = "encryptedContent")]
    pub encrypted_content: String,

    /// The nonce the post was encrypted with
    pub nonce: String,

    /// The username of the person that encrypted the post
    pub username: String,

    /// The public key of the person that encrypted the post
    #[serde(rename = "publicKey")]
    pub public_key: String,

    /// The ID of the post
    #[serde(rename = "postId")]
    pub post_id: i32,

    /// The time the post was made
    #[serde(rename = "timePosted")]
    pub time_posted: NaiveDateTime,

    /// The encrypted public key used to encrypt the post
    #[serde(rename = "encryptedPublicKey")]
    pub encrypted_public_key: String,

    /// The nonce used to decrypt the encrypted public key
    #[serde(rename = "encryptedPublicKeyNonce")]
    pub encrypted_public_key_nonce: String,
}

/// Represents the post body for responses from the POST endpoint
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Queryable, Serialize)]
pub struct NoaResponse {
    /// The post data
    pub post: PostResponse,

    /// The encrypted key to access the post data
    #[serde(rename = "encryptedSecretKey")]
    pub encrypted_secret_key: String,

    /// The nonce for the encrypted secret key
    pub nonce: String,

    /// Each username permitted to see the post.
    #[serde(rename = "allReaders")]
    pub all_readers: Vec<String>,

}

/// Represents a response from the NOA endpoint
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Queryable, Serialize)]
pub struct NoaOuterResponse {
    pub noas: Vec<NoaResponse>,
    pub pages: i64,
}

/// Represents the information needed to edit a post
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Queryable, Serialize)]
pub struct PostPutData {
    /// The ID of the post to patch
    #[serde(rename = "postId")]
    pub post_id: i32,

    /// The proof of authentication
    pub proof: String,

    /// The new encrypted content for the post
    #[serde(rename = "newContent")]
    pub new_content: String,

    /// The new nonce used to encrypt the content
    #[serde(rename = "newNonce")]
    pub new_nonce: String,
}
//! This program specifies a reference implementation of the back end of a
//! social media service that has no server-side knowledge of its users
//! authentication credentials.
#![feature(proc_macro_hygiene, decl_macro)]
#![warn(missing_docs, missing_debug_implementations)]

// Required due to poor support of 2018 edition by diesel.
#[macro_use]
extern crate diesel;

pub mod database;
pub mod models;
pub mod routes;
pub mod schema;

use crate::routes::*;
use database::CoreDbConn;
use rocket::routes;
use rocket_contrib::serve::StaticFiles;
use sodiumoxide::crypto::box_ as pkc;
use std::env::args;

/// The number of seconds after which a users authentication should timeout
const TIMEOUT_SECONDS: i64 = 3600;

/// Initialises the program, including loading the private and public keys which
/// are used by the server from an included file, an example for which can be
/// seen in `server_keys.example`.
fn main() {
    sodiumoxide::init().unwrap();
    let (server_public, server_secret) = pkc::gen_keypair();
    println!(
        "Starting Server With: {:?} / {:?}",
        base64::encode(&server_secret.0),
        &server_secret.0
    );
    rocket::ignite()
        .manage(server_public)
        .manage(server_secret)
        .attach(CoreDbConn::fairing())
        .mount("/_", routes![server_public_key::get, auth::get, auth::post, user::get, user::post])
        .mount("/", StaticFiles::from(args().skip(1).next().unwrap_or("static".to_string())))
        .launch();
}

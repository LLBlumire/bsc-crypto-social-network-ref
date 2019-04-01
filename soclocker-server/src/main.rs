#![feature(proc_macro_hygiene, decl_macro)]

// Required due to poor support of 2018 edition by diesel.
#[macro_use]
extern crate diesel;

mod database;
mod models;
mod routes;
mod schema;

use crate::routes::*;
use database::CoreDbConn;
use rocket::routes;
use rocket_contrib::serve::StaticFiles;
use sodiumoxide::crypto::box_ as pkc;
use std::env::args;

/// The number of seconds after which a users authentication should timeout
const TIMEOUT_SECONDS: i64 = 3600;

/// Initialises the program, including loading the private and public keys which are used by the
/// server.
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

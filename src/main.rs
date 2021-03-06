#![allow(dead_code)]

extern crate iron;
extern crate time;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;
extern crate bodyparser;
extern crate persistent;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate validator_derive;
extern crate validator;
#[macro_use]
extern crate serde_derive;
extern crate iron_json_response as ijr;
extern crate r2d2_redis;
extern crate jsonwebtoken as jwt;
extern crate iron_auth;

mod api;
mod db;
mod utils;

const PORT: i32 = 8000;

// TODO: Parse config from env.
fn main() {
    println!("Start listening on port {}...", PORT);
    api::start_listening(PORT);
}

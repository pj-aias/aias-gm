#[macro_use]
extern crate rbatis;

use crate::challenge::generate_challenge;
use crate::combine::generate_combined_pubkey;
use crate::gm::init_gm;
use crate::issue::issue_member;
use crate::pubkey::pubkey;
use actix_session::CookieSession;
use actix_web::HttpResponse;
use actix_web::{web, App, HttpServer};

use rand::Rng;

mod db;
mod gm;
mod handlers;
mod utils;

#[cfg(test)]
mod tests;

use crate::handlers::*;
use std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let key: [u8; 32] = rng.gen();

    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::private(&key).secure(true))
            .route("/", web::get().to(test_root))
            .route("/pubkey", web::post().to(pubkey))
            .route("/combine", web::post().to(generate_combined_pubkey))
            .route("/challenge", web::get().to(generate_challenge))
            .route("/issue", web::post().to(issue_member))
    })
    .workers(128)
    .shutdown_timeout(60)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

pub async fn test_root() -> Result<HttpResponse, actix_web::Error> {
    HttpResponse::Ok().json("ok").await
}

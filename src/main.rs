#[macro_use]
extern crate rbatis;

use crate::combine::generate_combined_pubkey;
use crate::gm::init_gm;
use crate::pubkey::pubkey;
use actix_session::CookieSession;
use actix_web::{web, App, HttpServer};

use rand::Rng;

mod db;
mod gm;
mod handlers;
mod tests;
mod utils;

use crate::handlers::*;
use std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let mut key = [0; 32];
    let mut rng = rand::thread_rng();

    for i in 0..32 {
        key[i] = rng.gen();
    }

    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::private(&key).secure(true))
            .route("/pubkey", web::post().to(pubkey))
            .route("/req_sign", web::post().to(generate_combined_pubkey))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

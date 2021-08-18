#[macro_use]
use actix_session::CookieSession;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use std::sync::Mutex;

use rand::Rng;

mod handler;
mod tests;

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
            .route("/pubkey", web::post().to(handler::pubkey))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

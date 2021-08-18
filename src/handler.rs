use actix_session::Session;
use actix_web::Error as WebError;
use actix_web::{web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetPubkeyReq {
    opener: Vec<String>,
}

pub async fn pubkey(phone_number: web::Json<GetPubkeyReq>) -> impl Responder {
    println!("hello");
    HttpResponse::Ok().body("Hello world")
}

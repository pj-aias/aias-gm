use actix_session::Session;
use actix_web::Error as WebError;
use actix_web::{web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetPubkeyReq {
    opener: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GetPubkeyResp {
    pubkey: String,
}

pub async fn pubkey(_openers: web::Json<GetPubkeyReq>) -> impl Responder {
    println!("hello");
    HttpResponse::Ok()
        .json(GetPubkeyResp {
            pubkey: "".to_string(),
        })
        .await
}

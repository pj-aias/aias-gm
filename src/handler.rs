use crate::db::Pubkey;
use crate::rbatis::executor::Executor;
use actix_session::Session;
use actix_web::Error as WebError;
use actix_web::{web, HttpResponse, Responder};

use crate::rbatis::crud::CRUD;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetPubkeyReq {
    pub openers: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GetPubkeyResp {
    pub pubkey: String,
}

use crate::db;

pub async fn pubkey(openers: web::Json<GetPubkeyReq>) -> impl Responder {
    println!("hello");

    let mut joined_openers = String::new();

    for opener in &openers.openers {
        joined_openers += opener;
    }

    let rb = db::init_db().await;

    let pubkey: Pubkey = match rb.fetch_by_column("openers", &joined_openers).await {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::Unauthorized().await;
        }
    };

    HttpResponse::Ok()
        .json(GetPubkeyResp {
            pubkey: pubkey.pubkey.expect("pubkey is None").to_string(),
        })
        .await
}

use crate::db::Credential;
use crate::rbatis::executor::Executor;
use actix_session::Session;
use actix_web::Error as WebError;
use actix_web::{web, HttpResponse, Responder};
use distributed_bss::opener::{Opener, OpenerId};
use rand::thread_rng;

use std::env;

use crate::open::g1_to_str;

use crate::open;
use bls12_381::G1Projective;

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

#[derive(Deserialize, Serialize)]
pub struct GetSignedKeyReq {
    pub pubkey: String,
    pub openers: Vec<String>,
}

use crate::db;

pub async fn pubkey(openers: web::Json<GetPubkeyReq>) -> Result<HttpResponse, actix_web::Error> {
    println!("hello");

    let mut joined_openers = String::new();
    if openers.openers.len() > 3 {
        return HttpResponse::BadRequest().await;
    }

    let rb = db::init_db().await;

    let pubkey: String = match rb
        .fetch_by_column::<Credential, String>("openers", &joined_openers)
        .await
    {
        Ok(cred) => cred.pubkey.unwrap(),
        Err(_) => open::gen_pubkey(&openers.openers, &rb).await,
    };

    HttpResponse::Ok()
        .json(GetPubkeyResp { pubkey: pubkey })
        .await
}

// pub async fn generate_signed_pubkey(openers: web::Json<GetPubkeyReq>) -> impl Responder {
//     // let pubkey =
// }

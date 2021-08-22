use crate::db::Credential;
use crate::init_opener;
use actix_web::{web, HttpResponse};
use distributed_bss::opener::OpenerId;
use rand::thread_rng;

use crate::open;

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
    let joined_openers = String::new();
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

pub async fn generate_signed_pubkey(
    req: web::Json<open::GenPubkeyReq>,
) -> Result<HttpResponse, actix_web::Error> {
    let unsigned_pubkey = req.unsigned_pubkey;
    let mut rng = thread_rng();

    let opener = init_opener(OpenerId::One, &mut rng).await;
    let unsigned_pubkey = distributed_bss::OPK {
        pubkey: unsigned_pubkey,
    };

    let signed_pubkey = opener.gen_pubkey(&unsigned_pubkey);

    HttpResponse::Ok()
        .json(open::GenPubkeyResp { signed_pubkey })
        .await
}

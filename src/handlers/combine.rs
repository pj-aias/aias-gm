use crate::gm::init_gm_from_domains;
use crate::gm::CombinedGPKWithoutPartials;
use crate::init_gm;
use crate::utils::encode;
use crate::utils::joined_domains;
use crate::utils::verify;
use crate::utils::verify_issuer_cert;
use actix_session::Session;
use actix_web::{web, HttpResponse};
use bls12_381::G2Projective;
use distributed_bss::gm::CombinedPubkey;
use distributed_bss::gm::GMId;
use distributed_bss::PartialUSK;
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;
use rbatis::crud::CRUD;

use crate::db;
use crate::gm;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SignPubkeyReq {
    pub domains: Vec<String>,
    pub unsigned_pubkey: CombinedPubkey,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SignPubkeyResp {
    pub signed_pubkey: CombinedPubkey,
}

pub async fn generate_combined_pubkey(
    req: web::Json<SignPubkeyReq>,
) -> Result<HttpResponse, actix_web::Error> {
    let unsigned_pubkey = req.unsigned_pubkey;
    let mut rng = thread_rng();

    let gm = init_gm(GMId::One, &mut rng).await;

    let signed_pubkey = gm.gen_combined_pubkey(&unsigned_pubkey);

    HttpResponse::Ok()
        .json(SignPubkeyResp { signed_pubkey })
        .await
}

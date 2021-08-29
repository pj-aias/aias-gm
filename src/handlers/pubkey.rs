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

use crate::gm;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetPubkeyReq {
    pub domains: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GetPubkeyResp {
    pub combined: CombinedGPKWithoutPartials,
    pub partial: G2Projective,
}

use crate::db;

pub async fn pubkey(domains: web::Json<GetPubkeyReq>) -> Result<HttpResponse, actix_web::Error> {
    if domains.domains.len() != 3 {
        return HttpResponse::BadRequest().await;
    }

    let rb = db::init_db().await;
    let mut rng = thread_rng();

    let gm = init_gm_from_domains(&domains.domains, &mut rng).await;

    let combined = gm::gen_pubkey(&gm, &domains.domains, &rb)
        .await
        .expect("errro generate pubkey");

    let partial = gm.gpk.omega;

    HttpResponse::Ok()
        .json(GetPubkeyResp { combined, partial })
        .await
}

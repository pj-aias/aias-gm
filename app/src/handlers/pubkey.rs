use actix_web::{web, HttpResponse};
use bls12_381::G2Projective;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use crate::gm;
use crate::gm::init_gm_from_domains;
use crate::gm::CombinedGPKWithoutPartials;

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

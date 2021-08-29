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
pub struct GenerateChallengeResp {
    pub nonce: String,
}

pub async fn generate_challenge(session: Session) -> Result<HttpResponse, actix_web::Error> {
    let nonce: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    match session.set("nonce", nonce.clone()) {
        Ok(_) => {
            HttpResponse::Ok()
                .json(GenerateChallengeResp { nonce })
                .await
        }
        Err(_) => panic!("todo fix"),
    }
}

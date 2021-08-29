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

#[derive(Deserialize, Serialize)]
pub struct GetSignedKeyReq {
    pub pubkey: String,
    pub domains: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct IssueMemberResp {
    pub partical_usk: PartialUSK,
}

#[derive(Deserialize, Serialize)]
pub struct IssueMemberReq {
    pub cert: String,
    pub signature: String,
    pub pubkey: String,
    pub domains: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GenerateChallengeResp {
    pub nonce: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SignPubkeyReq {
    pub domains: Vec<String>,
    pub unsigned_pubkey: CombinedPubkey,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SignPubkeyResp {
    pub signed_pubkey: CombinedPubkey,
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

pub async fn issue_member(
    req: web::Json<IssueMemberReq>,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let nonce = session.get::<String>("nonce")?.expect("nonce is not found");

    if !verify_issuer_cert(&req.cert, &req.pubkey) || !verify(&req.signature, &nonce, &req.pubkey) {
        panic!();
    }

    let rb = db::init_db().await;
    let mut rng = thread_rng();

    let gm = init_gm_from_domains(&req.domains, &mut rng).await;

    let combined = gm::gen_pubkey(&gm, &req.domains, &rb)
        .await
        .expect("errro generate pubkey");

    let partial = gm.gpk.omega;

    let usk = gm.issue_member(&mut rng);
    let usk = encode(&usk);

    let cert = encode(&req.cert.clone());

    rb.save(
        &db::Member {
            id: None,
            domains: Some(joined_domains(&req.domains)),
            usk: Some(usk),
            cert: Some(cert),
        },
        &[],
    )
    .await
    .expect("Error DB");

    HttpResponse::Ok()
        .json(GetPubkeyResp { combined, partial })
        .await
}

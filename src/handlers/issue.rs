use crate::gm::init_gm_from_domains;


use crate::utils::encode;
use crate::utils::joined_domains;
use crate::utils::verify;
use crate::utils::verify_issuer_cert;
use actix_session::Session;
use actix_web::{web, HttpResponse};



use distributed_bss::PartialUSK;

use rand::thread_rng;

use rbatis::crud::CRUD;

use crate::db;
use crate::gm;

use serde::{Deserialize, Serialize};

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

    let _combined = gm::gen_pubkey(&gm, &req.domains, &rb)
        .await
        .expect("errro generate pubkey");

    let _partial = gm.gpk.omega;

    let partical_usk = gm.issue_member(&mut rng);
    let encode_usk = encode(&partical_usk);

    let cert = encode(&req.cert.clone());

    rb.save(
        &db::Member {
            id: None,
            domains: Some(joined_domains(&req.domains)),
            usk: Some(encode_usk),
            cert: Some(cert),
        },
        &[],
    )
    .await
    .expect("Error DB");

    HttpResponse::Ok().json(IssueMemberResp { partical_usk }).await
}

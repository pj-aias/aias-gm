use crate::gm::init_gm_from_domains;

use crate::utils::joined_domains;
use crate::utils::verify;
use crate::utils::verify_issuer_cert;
use actix_session::Session;
use actix_web::Error as WebError;
use actix_web::{web, HttpResponse};

use distributed_bss::PartialUSK;

use rand::thread_rng;

use rbatis::crud::CRUD;

use crate::db;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct IssueMemberResp {
    pub partial_usk: PartialUSK,
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
) -> Result<HttpResponse, WebError> {
    let nonce = session.get::<String>("nonce")?.expect("nonce is not found");
    if !verify(&req.signature, &nonce, &req.pubkey) {
        return HttpResponse::Unauthorized().json({}).await;
    }

    if !verify_issuer_cert(&req.cert, &req.pubkey) {
        return HttpResponse::Unauthorized().json({}).await;
    }

    let rb = db::init_db().await;
    let mut rng = thread_rng();

    let gm = init_gm_from_domains(&req.domains, &mut rng).await;
    let partial_usk = gm.issue_member(&mut rng);

    // let encode_usk = encode(&partical_usk);

    // let cert = encode(&req.cert.clone());

    rb.save(
        &db::Member {
            id: None,
            domains: Some(joined_domains(&req.domains)),
            usk: Some("encode_usk".to_string()),
            cert: Some("cert".to_string()),
        },
        &[],
    )
    .await
    .expect("Error DB");

    HttpResponse::Ok()
        .json(IssueMemberResp { partial_usk })
        .await
}

use actix_web::{web, HttpResponse};
use distributed_bss::gm::CombinedPubkey;
use distributed_bss::gm::GMId;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use crate::init_gm;

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

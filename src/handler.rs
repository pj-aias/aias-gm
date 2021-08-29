use crate::db::Credential;
use crate::gm::init_gm_from_domains;
use crate::init_gm;
use crate::utils::joined_gms;
use actix_web::{web, HttpResponse};
use distributed_bss::gm::GMId;
use rand::thread_rng;

use distributed_bss::gm::CombinedPubkey;

use crate::gm;

use crate::rbatis::crud::CRUD;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetPubkeyReq {
    pub domains: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GetPubkeyResp {
    pub pubkey: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetSignedKeyReq {
    pub pubkey: String,
    pub domains: Vec<String>,
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
    let joined_domains = joined_gms(&domains.domains);

    let pubkey: String = match rb
        .fetch_by_column::<Credential, String>("gms", &joined_domains)
        .await
    {
        Ok(cred) => cred.pubkey.unwrap(),
        Err(_) => gm::gen_pubkey(&gm, &domains.domains, &rb).await,
    };

    let usk = gm.issue_member(&mut rng);

    HttpResponse::Ok()
        .json(GetPubkeyResp { pubkey: pubkey })
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

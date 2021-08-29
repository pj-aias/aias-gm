use crate::combine::SignPubkeyResp;
use crate::combine::SignPubkeyReq;
use crate::rbatis::crud::CRUD;
use crate::utils::encode;
use crate::utils::get_gm_index_from_domains;
use crate::utils::gm_id;
use crate::utils::joined_domains;
use actix_web::client::Client;
use bls12_381::G1Projective;
use bls12_381::Scalar;
use distributed_bss::gm::CombinedPubkey;
use distributed_bss::gm::{GMId, GM};
use rand::Rng;
use rbatis::rbatis::Rbatis;
use std::env;

// use crate::handlers::*;

// SignPubkeyReq;

use crate::db;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CombinedGPKWithoutPartials {
    pub h: G1Projective,
    pub u: G1Projective,
    pub v: G1Projective,
}

/// init GM
pub async fn init_gm(id: GMId, rng: &mut impl Rng) -> GM {
    let sec1 = env::var("AIAS_OPENER_SECRET_KEY1").unwrap_or("".to_string());
    let sec2 = env::var("AIAS_OPENER_SECRET_KEY2").unwrap_or("".to_string());

    if sec1.is_empty() || sec2.is_empty() {
        // generate new gm
        let gm = GM::random(id, rng);
        let sec1 = rmp_serde::to_vec(&gm.gsk.xi).expect("MessagePack encode error");
        let sec1 = base64::encode(&sec1);

        let sec2 = rmp_serde::to_vec(&gm.gsk.gamma).expect("MessagePack encode error");
        let sec2 = base64::encode(&sec2);

        env::set_var("AIAS_OPENER_SECRET_KEY1", sec1);
        env::set_var("AIAS_OPENER_SECRET_KEY2", sec2);

        return gm;
    } else {
        // init gm from enviroment data
        let sec1 = base64::decode(&sec1).expect("privkey decode error");
        let sec1: Scalar = rmp_serde::from_slice(&sec1).expect("MessagePack decode error");

        let sec2 = base64::decode(&sec2).expect("privkey decode error");
        let sec2: Scalar = rmp_serde::from_slice(&sec2).expect("MessagePack decode error");

        return GM::new(id, &sec1, &sec2);
    }
}

pub async fn init_gm_from_domains(domains: &Vec<String>, rng: &mut impl Rng) -> GM {
    let self_index = get_gm_index_from_domains(domains) as u8;

    let gm_id = gm_id(self_index).unwrap();
    init_gm(gm_id, rng).await
}

pub async fn gen_pubkey(
    gm: &GM,
    domains: &Vec<String>,
    rb: &Rbatis,
) -> Result<CombinedGPKWithoutPartials, String> {
    let mut domains = domains.clone();

    let unsigned_pubkey = gm.gpk.h;
    let joined_domains = joined_domains(&domains);

    let pubkeys1 = communicate_to_gen_pubkey(gm, &domains, &unsigned_pubkey).await;

    domains.reverse();
    let pubkeys2 = communicate_to_gen_pubkey(gm, &domains, &unsigned_pubkey).await;

    if pubkeys1[1] != pubkeys2[1] {
        return Err("wrong combined pubkey generated".to_string());
    }

    let pubkey = CombinedGPKWithoutPartials {
        h: pubkeys1[1],
        u: pubkeys1[0],
        v: pubkeys2[0],
    };

    let encoded = encode(&pubkey);

    rb.save(
        &db::PublicKey {
            id: None,
            domains: Some(joined_domains),
            pubkey: Some(encoded),
            gm_id: Some(gm.id as u8),
        },
        &[],
    )
    .await
    .expect("Error DB");

    Ok(pubkey)
}

pub async fn communicate_to_gen_pubkey(
    gm: &GM,
    domains: &Vec<String>,
    unsigned_pubkey: &CombinedPubkey,
) -> Vec<CombinedPubkey> {
    let mut pubkeys = vec![];

    let mut unsigned_pubkey = *unsigned_pubkey;
    for (index, gm_domain) in domains.iter().enumerate() {
        if gm.id as usize == index {
            continue;
        }

        let req = SignPubkeyReq {
            domains: domains.clone(),
            unsigned_pubkey: unsigned_pubkey,
        };

        let url = format!("http://{}/req_sign", gm_domain);

        let client = Client::new();

        let resp = client
            .post(url)
            .send_json(&req)
            .await
            .expect("request error")
            .json::<SignPubkeyResp>()
            .await
            .expect("parse error");

        unsigned_pubkey = resp.signed_pubkey;
        pubkeys.push(unsigned_pubkey);
    }

    pubkeys
}

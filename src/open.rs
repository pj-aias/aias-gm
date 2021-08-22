use distributed_bss::opener::{Opener, OpenerId};
use rand::thread_rng;
use rbatis::rbatis::Rbatis;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::env;

use bls12_381::G1Projective;

use crate::db;

#[derive(Clone, Serialize, Deserialize)]
pub struct GenPubkeyReq {
    pub openers: Vec<String>,
    pub unsigned_pubkey: G1Projective,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GenPubkeyResp {
    pub unsigned_pubkey: G1Projective,
}

pub async fn gen_pubkey(openers: &Vec<String>, rb: &Rbatis) -> String {
    let domain = env::var("AIAS_OPENER_DOMAIN").expect("not set AIAS_OPENER_DOMAIN");
    let index = openers
        .iter()
        .position(|d| d == &domain)
        .expect("domain is invalid");

    let joined_openers = joined_openers(&openers);

    let mut rng = thread_rng();
    let opener_id = opener_id(index as u8).unwrap();

    let opener = Opener::random(opener_id, &mut rng);
    let privkey = rmp_serde::to_vec(&opener.osk).expect("MessagePack encode error");
    let privkey = base64::encode(&privkey);

    let mut unsigned_pubkey = opener.opk.pubkey;

    for opener_domain in openers {
        if opener.id as usize == index {
            continue;
        }

        let req = GenPubkeyReq {
            openers: openers.clone(),
            unsigned_pubkey,
        };

        let req = serde_json::to_string(&req).expect("Json encode error");

        let url = format!("https://{}/request_sign", opener_domain);

        let client = reqwest::Client::new();

        let resp = client
            .post(url)
            .header("Content-Type", "text/json")
            .body(req.clone())
            .send()
            .await
            .expect("request error")
            .json::<GenPubkeyResp>()
            .await
            .expect("parse error");

        unsigned_pubkey = resp.unsigned_pubkey;
    }

    let pubkey = g1_to_str(&unsigned_pubkey);

    db::save(
        &rb,
        &db::Credential {
            id: None,
            openers: Some(joined_openers),
            pubkey: Some(pubkey.clone()),
            opener_id: Some(index.try_into().unwrap()),
        },
    )
    .await;

    pubkey
}

pub fn g1_to_str(point: &G1Projective) -> String {
    let point = rmp_serde::to_vec(&point).expect("rmp encode error");
    let point = base64::encode(&point);

    return point;
}

pub fn joined_openers(openers: &Vec<String>) -> String {
    let mut joined_openers = String::new();

    for opener in openers {
        joined_openers += &(opener.to_owned() + ",");
    }

    return joined_openers;
}

pub fn opener_id(num: u8) -> Option<OpenerId> {
    let res = match num % 4 {
        1 => OpenerId::One,
        2 => OpenerId::Two,
        3 => OpenerId::Three,
        _ => return None,
    };

    Some(res)
}

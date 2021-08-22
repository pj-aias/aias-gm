use actix_web::client::Client;
use bls12_381::Scalar;
use distributed_bss::opener::{Opener, OpenerId};
use rand::thread_rng;
use rand::Rng;
use rbatis::rbatis::Rbatis;
use serde::{Deserialize, Serialize};
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
    pub signed_pubkey: G1Projective,
}

pub async fn init_opener(id: OpenerId, rng: &mut impl Rng) -> Opener {
    match env::var("AIAS_OPENER_SECRET_KEY") {
        Ok(privkey) => {
            let privkey = base64::decode(&privkey).expect("privkey decode error");
            let privkey: Scalar =
                rmp_serde::from_slice(&privkey).expect("MessagePack decode error");

            return Opener::new(id, &privkey);
        }
        Err(_) => {
            let opener = Opener::random(id, rng);
            let privkey = rmp_serde::to_vec(&opener.osk.xi).expect("MessagePack encode error");
            let privkey = base64::encode(&privkey);

            env::set_var("AIAS_OPENER_SECRET_KEY", privkey);
            return opener;
        }
    };
}

pub async fn gen_pubkey(openers: &Vec<String>, rb: &Rbatis) -> String {
    let domain = env::var("AIAS_OPENER_DOMAIN").expect("not set AIAS_OPENER_DOMAIN");
    let index = openers
        .iter()
        .position(|d| d == &domain)
        .expect("domain is invalid");

    let index = index + 1;

    let joined_openers = joined_openers(&openers);

    let mut rng = thread_rng();
    let opener_id = opener_id((index as u8) + 1).unwrap();

    let opener = init_opener(opener_id, &mut rng).await;

    let mut unsigned_pubkey = opener.opk.pubkey;

    println!("tests: {:?}", openers);

    for (index, opener_domain) in openers.iter().enumerate() {
        if opener.id as usize == index {
            println!("tests: 1");

            continue;
        }

        let req = GenPubkeyReq {
            openers: openers.clone(),
            unsigned_pubkey,
        };

        let url = format!("http://{}/req_sign", opener_domain);

        let client = Client::new();
        println!("tests: {}", url.clone());

        let resp = client
            .post(url)
            .send_json(&req)
            .await
            .expect("request error")
            .json::<GenPubkeyResp>()
            .await
            .expect("parse error");

        unsigned_pubkey = resp.signed_pubkey;
    }

    let pubkey = g1_to_str(&unsigned_pubkey);

    db::save(
        &rb,
        &db::Credential {
            id: None,
            openers: Some(joined_openers),
            pubkey: Some(pubkey.clone()),
            opener_id: Some(index as u8),
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

pub fn str_to_g1(point: &String) -> G1Projective {
    let point = base64::decode(&point).expect("base64 decode error");
    let point = rmp_serde::from_slice(&point).expect("rmp decode error");

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

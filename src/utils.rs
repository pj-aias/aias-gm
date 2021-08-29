// use crate::gm::CombinedGPKWithoutPartials;
use distributed_bss::gm::GMId;
use serde::Serialize;

use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::sign::Verifier;

use std::env;

pub fn encode<T>(point: &T) -> String
where
    T: Serialize,
{
    let point = rmp_serde::to_vec(&point).expect("rmp encode error");
    let point = base64::encode(&point);

    return point;
}

// pub fn decode_to_combined(point: &String) -> CombinedGPKWithoutPartials {
//     let point = base64::decode(point).expect("base64 decode error");
//     let point = rmp_serde::from_slice(&point).expect("rmp decode error");

//     return point;
// }

pub fn joined_domains(gms: &Vec<String>) -> String {
    let mut joined_domains = String::new();

    for gm in gms {
        joined_domains += &(gm.to_owned() + ",");
    }

    return joined_domains;
}

pub fn gm_id(num: u8) -> Option<GMId> {
    let res = match num % 4 {
        1 => GMId::One,
        2 => GMId::Two,
        3 => GMId::Three,
        _ => return None,
    };

    Some(res)
}

pub fn get_gm_index_from_domains(gms: &[String]) -> usize {
    let domain = env::var("AIAS_OPENER_DOMAIN").expect("not set AIAS_OPENER_DOMAIN");

    let index = gms
        .iter()
        .position(|d| d == &domain)
        .expect("domain is invalid");

    return index + 1;
}

pub fn verify(signature: &String, msg: &String, pubkey: &String) -> bool {
    let keypair = PKey::public_key_from_pem(&pubkey.as_bytes()).expect("pem decode error");
    let mut verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
    verifier.update(pubkey.as_bytes()).unwrap();
    verifier.verify(signature.as_bytes()).unwrap()
}

pub fn verify_issuer_cert(cert: &String, user_pubkey: &String) -> bool {
    let issuer_pubkey = env::var("AIAS_ISSUER_PUBKEY").expect("pem is not found");
    let keypair = PKey::public_key_from_pem(&issuer_pubkey.as_bytes()).expect("pem decode error");

    verify(cert, user_pubkey, &issuer_pubkey)
}

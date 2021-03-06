// use crate::gm::CombinedGPKWithoutPartials;
use distributed_bss::gm::GMId;
use openssl::pkey::Public;
use openssl::rsa::Rsa;
use serde::Serialize;

use openssl::hash::MessageDigest;
use openssl::pkey::PKey;

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

pub fn get_pubkey(pubkey: &String) -> PKey<Public> {
    let is_pkcs1 = pubkey.find("RSA PUBLIC KEY");

    if is_pkcs1 == None {
        PKey::public_key_from_pem(&pubkey.as_bytes()).expect("pem decode error")
    } else {
        let rsakey = Rsa::public_key_from_pem_pkcs1(&pubkey.as_bytes()).expect("pkcs decode error");
        PKey::from_rsa(rsakey).expect("error convert pkcs")
    }
}

pub fn verify(signature: &String, msg: &String, pubkey: &String) -> bool {
    println!("signature: {}\npubkey: {}\nmsg: {}", signature, pubkey, msg);

    let bin_signature = base64::decode(signature).expect("base64 decode error");

    let pubkey = get_pubkey(pubkey);
    let mut verifier = Verifier::new(MessageDigest::sha256(), &pubkey).unwrap();
    verifier.update(msg.as_bytes()).unwrap();

    verifier.verify(&bin_signature).unwrap()
}

pub fn verify_issuer_cert(cert: &String, user_pubkey: &String) -> bool {
    let issuer_pubkey = env::var("AIAS_ISSUER_PUBKEY").expect("pem is not found");
    verify(cert, user_pubkey, &issuer_pubkey)
}

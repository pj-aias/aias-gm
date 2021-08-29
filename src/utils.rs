// use crate::gm::CombinedGPKWithoutPartials;
use distributed_bss::gm::GMId;
use serde::Serialize;

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

pub fn joined_gms(gms: &Vec<String>) -> String {
    let mut joined_gms = String::new();

    for gm in gms {
        joined_gms += &(gm.to_owned() + ",");
    }

    return joined_gms;
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

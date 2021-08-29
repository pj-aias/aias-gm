use actix_session::Session;
use actix_web::HttpResponse;

use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GenerateChallengeResp {
    pub nonce: String,
}

pub async fn generate_challenge(session: Session) -> Result<HttpResponse, actix_web::Error> {
    let nonce: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    match session.set("nonce", nonce.clone()) {
        Ok(_) => {
            HttpResponse::Ok()
                .json(GenerateChallengeResp { nonce })
                .await
        }
        Err(_) => panic!("todo fix"),
    }
}

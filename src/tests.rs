use crate::issue_member;
use crate::generate_challenge;
use crate::combine::generate_combined_pubkey;

use crate::pubkey::pubkey;
use crate::pubkey::GetPubkeyReq;
use crate::pubkey::GetPubkeyResp;
use actix_session::CookieSession;
use actix_web::client::Client;
use actix_web::HttpServer;
use actix_web::{web, App};
use distributed_bss::gm::GMId;
use rand::thread_rng;

use crate::gm;

use std::process::Command;

#[actix_rt::test]
async fn test_app() {
    Command::new("touch").args(&["aias.db"]).output().unwrap();

    let gms = GetPubkeyReq {
        domains: [
            "localhost:8080".to_string(),
            "localhost:8080".to_string(),
            "localhost:8080".to_string(),
        ]
        .to_vec(),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::private(&[0; 32]).secure(true))
            .route("/pubkey", web::post().to(pubkey))
            .route("/combine", web::post().to(generate_combined_pubkey))
            .route("/challenge", web::get().to(generate_challenge))
            .route("/issue", web::post().to(issue_member))
    })
    .bind("0.0.0.0:8080")
    .expect("run server error")
    .run();

    let client = Client::new();

    let resp = client
        .post("http://localhost:8080/pubkey")
        .send_json(&gms)
        .await
        .expect("request error")
        .json::<GetPubkeyResp>()
        .await
        .unwrap();

    let h = resp.combined.h;

    let mut rng = thread_rng();
    let gm = gm::init_gm(GMId::One, &mut rng).await;
    let expect = gm.gpk.h * gm.gsk.xi * gm.gsk.xi;

    assert_eq!(h, expect);
}

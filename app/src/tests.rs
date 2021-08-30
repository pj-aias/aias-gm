use crate::challenge::GenerateChallengeResp;
use crate::combine::generate_combined_pubkey;
use crate::generate_challenge;
use crate::issue::IssueMemberReq;
use crate::issue_member;
use crate::utils::verify;
use crate::utils::verify_issuer_cert;

use crate::gm;
use crate::pubkey::pubkey;
use crate::pubkey::GetPubkeyReq;
use crate::pubkey::GetPubkeyResp;
use actix_session::CookieSession;
use actix_web::client::Client;
use actix_web::HttpServer;
use actix_web::{test, web, App};
use distributed_bss::gm::GMId;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::sign::Signer;
use rand::thread_rng;
use serde_json;

use std::env;
use std::process::Command;

#[actix_rt::test]
async fn test_pubkey() {
    Command::new("touch").args(&["aias.db"]).output().unwrap();

    let gms = GetPubkeyReq {
        domains: [
            "localhost:8080".to_string(),
            "localhost:8080".to_string(),
            "localhost:8080".to_string(),
        ]
        .to_vec(),
    };

    let server = HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::private(&[0; 32]).secure(true))
            .route("/pubkey", web::post().to(pubkey))
            .route("/combine", web::post().to(generate_combined_pubkey))
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

    server.stop(true).await;
}

#[actix_rt::test]
async fn test_generate_usk() {
    Command::new("touch").args(&["aias.db"]).output().unwrap();

    let domains = [
        "localhost:8080".to_string(),
        "localhost:8080".to_string(),
        "localhost:8080".to_string(),
    ]
    .to_vec();

    let mut app = test::init_service(
        App::new()
            .wrap(CookieSession::private(&[0; 32]).secure(true))
            .route("/challenge", web::get().to(generate_challenge))
            .route("/issue", web::post().to(issue_member)),
    )
    .await;

    let req = test::TestRequest::get().uri("/challenge").to_request();
    let resp = test::call_service(&mut app, req).await;

    let cookie = resp
        .response()
        .cookies()
        .find(|c| c.name() == "actix-session")
        .expect("failed to get id from response's session");

    let req = test::TestRequest::post()
        .uri("/issue")
        .cookie(cookie.clone());

    println!("cookie: {}", cookie);

    let body = test::read_body(resp).await;
    let body = String::from_utf8(body.to_vec()).unwrap();

    let nonce: GenerateChallengeResp = serde_json::from_str(&body).unwrap();
    let nonce = nonce.nonce;

    let payload = generate_test_issuer_req(&nonce, &domains);

    println!("nonce: {}", nonce);

    let req = req.set_json(&payload).to_request();

    let resp = test::call_service(&mut app, req).await;

    let body = test::read_body(resp).await;
    let body = String::from_utf8(body.to_vec()).unwrap();

    println!("result : {:}", body);
}

// #[test]
// fn test_generate_test_issuer_req() {
//     let nonce = "hogehoge".to_string();

//     let domains = [
//         "localhost:8081".to_string(),
//         "localhost:8081".to_string(),
//         "localhost:8081".to_string(),
//     ]
//     .to_vec();

//     generate_test_issuer_req(&nonce, &domains);
// }

fn generate_test_issuer_req(nonce: &String, domains: &[String]) -> IssueMemberReq {
    // set up issuer
    let issuer_privkey = Rsa::generate(2048).unwrap();
    let issuer_pubkey = PKey::from_rsa(issuer_privkey).expect("key generation error");
    let issuer_pubkey_pem = issuer_pubkey.public_key_to_pem().unwrap();
    let issuer_pubkey_pem = base64::encode(&issuer_pubkey_pem);

    env::set_var("AIAS_ISSUER_PUBKEY", issuer_pubkey_pem);

    // set up user
    let user_privkey = Rsa::generate(2048).unwrap();
    let user_pubkey = PKey::from_rsa(user_privkey).expect("key generation error");
    let user_pubkey_pem = user_pubkey.public_key_to_pem().unwrap();
    let user_pubkey_pem = base64::encode(&user_pubkey_pem);

    // set up signature
    let mut signer = Signer::new(MessageDigest::sha256(), &user_pubkey).expect("sign error");
    signer.update(nonce.as_bytes()).expect("sign error");

    let signature = signer.sign_to_vec().expect("sign error");
    let signature = base64::encode(signature);

    assert!(verify(&signature, &nonce, &user_pubkey_pem));

    // set up cert
    let mut signer = Signer::new(MessageDigest::sha256(), &issuer_pubkey).expect("sign error");
    signer
        .update(&user_pubkey_pem.as_bytes())
        .expect("sign error");

    let cert = signer.sign_to_vec().expect("sign error");
    let cert = base64::encode(cert);

    assert!(verify_issuer_cert(&cert, &user_pubkey_pem));

    return IssueMemberReq {
        cert,
        signature,
        pubkey: user_pubkey_pem,
        domains: domains.to_vec(),
    };
}

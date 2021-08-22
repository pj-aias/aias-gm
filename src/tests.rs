use crate::handler::GetPubkeyReq;
use crate::handler::GetPubkeyResp;
use crate::open::str_to_g1;
use actix_session::CookieSession;
use actix_web::client::Client;
use actix_web::HttpServer;
use actix_web::{web, App};
use distributed_bss::opener::OpenerId;
use rand::thread_rng;

use crate::handler;
use crate::open;

#[actix_rt::test]
async fn test_app() {
    let openers = GetPubkeyReq {
        openers: [
            "localhost:8080".to_string(),
            "localhost:8080".to_string(),
            "localhost:8080".to_string(),
        ]
        .to_vec(),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::private(&[0; 32]).secure(true))
            .route("/pubkey", web::post().to(handler::pubkey))
            .route("/req_sign", web::post().to(handler::generate_signed_pubkey))
    })
    .bind("0.0.0.0:8080")
    .expect("run server error")
    .run();

    let client = Client::new();

    let resp = client
        .post("http://localhost:8080/pubkey")
        .send_json(&openers)
        .await
        .expect("request error")
        .json::<GetPubkeyResp>()
        .await
        .unwrap();

    let pubkey = resp.pubkey;
    let pubkey = str_to_g1(&pubkey);

    let mut rng = thread_rng();
    let opener = open::init_opener(OpenerId::One, &mut rng).await;
    let expect = opener.opk.pubkey * opener.osk.xi * opener.osk.xi;

    assert_eq!(pubkey, expect);
}

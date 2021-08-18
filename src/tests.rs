use actix_session::CookieSession;
use actix_web::{test, web, App};

use crate::handler;
use handler::GetPubkeyReq;

#[actix_rt::test]
async fn test() {
    let app = App::new()
        .wrap(CookieSession::private(&[0; 32]).secure(true))
        .route("/pubkey", web::post().to(handler::pubkey));

    let mut app = test::init_service(app).await;

    let openers = GetPubkeyReq {
        openers: [
            "a.example.com".to_string(),
            "b.example.com".to_string(),
            "c.example.com".to_string(),
        ]
        .to_vec(),
    };

    let openers = serde_json::to_string(&openers).unwrap();

    let req = test::TestRequest::post()
        .uri("/pubkey")
        .set_payload(openers)
        .header("Content-Type", "text/json")
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    let resp = resp.response();
}

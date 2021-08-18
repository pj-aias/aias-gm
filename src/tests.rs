use actix_session::CookieSession;
use actix_web::{test, web, App};

use crate::handler;

#[actix_rt::test]
async fn test() {
    let app = App::new()
        .wrap(CookieSession::private(&[0; 32]).secure(true))
        .route("/pubkey", web::post().to(handler::pubkey));
}

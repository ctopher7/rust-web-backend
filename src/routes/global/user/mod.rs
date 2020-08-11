mod protected_route;
use actix_web::web;

use crate::controllers::global::user::handler::*;


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/login/web", web::post().to(web_login))
            .route("/signup", web::post().to(sign_up))
            .route("/signup/check/email", web::post().to(check_email_exist))
            .configure(protected_route::routes)
    );
}
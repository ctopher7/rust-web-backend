use actix_web::web;

use crate::controllers::global::user::handler::*;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/login", web::post().to(login))
    );
}
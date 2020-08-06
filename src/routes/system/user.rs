use actix_web::web;

use crate::controllers::system::user::handler::*;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/verify/email", web::get().to(verify_email))
    );
}
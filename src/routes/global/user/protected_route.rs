use actix_web::web;

use crate::controllers::global::user::handler_protected::*;
use crate::middlewares::auth::{Auth,AuthType};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/protected")
        .wrap(Auth{
            classification:AuthType::JWT(vec![
                "customer".to_string(),
                "admin".to_string()
            ])
        })
        .route("/profile", web::get().to(get_profile))
        .route("/profile/update", web::post().to(update_profile))
        .route("/password/change",web::post().to(change_password))
    );
}
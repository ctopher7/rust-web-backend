mod user;

use actix_web::web;

use crate::middlewares::auth::{Auth,AuthType};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .wrap(Auth{classification:AuthType::JWT("admin")})
            .configure(user::routes)
    );
}
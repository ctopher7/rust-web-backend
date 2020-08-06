pub mod user;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/system")
            .configure(user::routes)
    );
}
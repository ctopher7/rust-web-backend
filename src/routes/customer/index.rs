use actix_web::web;
use crate::controllers::customer::index::*;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/customer")
            .route("/main", web::get().to(main))
    );
}
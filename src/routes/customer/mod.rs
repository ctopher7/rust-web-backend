mod index;

use actix_web::web;

use crate::middlewares::auth::{Auth,AuthType};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/customer")
            .wrap(Auth{classification:AuthType::JWT(vec!["customer".to_string()])})
            .configure(index::routes)
    );
}
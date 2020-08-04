mod customer;
mod admin;
mod global;

use actix_web::web;

use crate::middlewares::auth::{Auth,AuthType};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(Auth{classification:AuthType::APIKEY})
            .configure(customer::routes)
            .configure(admin::routes)
            .configure(global::routes)
    );
}
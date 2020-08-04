use actix_web::web::{get,ServiceConfig,scope,post};
use crate::controllers::admin::user::handler::*;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/user")
            .route("/create", post().to(create))
            .route("/all", get().to(all))
    );
}
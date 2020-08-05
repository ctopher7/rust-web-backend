use actix_web::web::{ServiceConfig,scope,post};
use crate::controllers::internal::user::handler::*;
use crate::middlewares::auth::{Auth,AuthType};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/user")
            .wrap(Auth{classification:AuthType::JWT(vec!["superadmin".to_string()])})
            .route("/create", post().to(create))
    );
}
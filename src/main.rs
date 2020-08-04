use actix_web::{App,HttpServer,web};

mod routes;
mod controllers;
mod utils;
mod db;
mod regex;
mod middlewares;

#[macro_use]
extern crate validator_derive;

#[derive(Debug)]
pub struct AppState{
    db_postgres: sqlx::PgPool
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    utils::cli::read_cli();

    let postgres_session = db::postgres::create_connection().await.unwrap();

    HttpServer::new(move ||
        App::new()
            .app_data(web::Data::new(AppState{
                db_postgres: postgres_session.clone(),
            }))
            .configure(routes::routes)
    )
        .workers(num_cpus::get())
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

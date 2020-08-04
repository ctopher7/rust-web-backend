use clap::{Arg, App};
use dotenv::from_filename;

pub fn read_cli(){
    let app = App::new("rust-app")
    .arg(Arg::with_name("environment")
        .short("e")
        .long("env")
        .value_name("ENV_NAME")
        .help("Load env file")
        .takes_value(true)
    )
    .get_matches();

    let env = app.value_of("environment").unwrap_or("production");

    match env{
        "development" | "dev" => { from_filename(".env.development").expect("load env error") ;},
        "production" | "prod" | _ => { from_filename(".env.production").expect("load env error") ;}
    }
}
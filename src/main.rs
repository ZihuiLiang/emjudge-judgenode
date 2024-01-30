use actix_web::{middleware::Logger, web, App, HttpServer};
use emjudge_judgenode::{service::{auth::{is_authed, Authentication}, info, test_data, tests}, settings::Settings};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let settings = match Settings::load_from_file("config/settings.toml", config::FileFormat::Toml) {
        Ok(settings) => settings,
        Err(error) => {
            println!("Error: {}", error);
            return Ok(());
        }
    };
    println!("settings: {:?}", settings);
    let url = settings.url.clone();
    let port = settings.port.clone();
    let workers = settings.actix_workers.clone();
    let max_json_limit = settings.max_json_limit.clone().as_bytes() as usize;
    HttpServer::new(move ||  {
        App::new()
            .app_data(web::JsonConfig::default().limit(max_json_limit))
            .app_data(web::Data::new(settings.clone()))
            .wrap(Logger::default())
            .wrap(Authentication)
            .service(is_authed)
            .service(web::scope("/test").service(tests::only_run).service(tests::run_and_eval).service(tests::ans_and_eval).service(tests::run_and_interact))
            .service(web::scope("/test_data").service(test_data::submit))
            .service(web::scope("/info").service(info::all).service(info::disk).service(info::cpu).service(info::mem).service(info::sys).service(info::language).service(info::languages))
    })
    .bind((url, port))?
    .workers(workers)
    .run()
    .await
}
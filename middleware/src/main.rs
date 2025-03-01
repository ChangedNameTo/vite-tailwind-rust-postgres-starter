mod authenticate_token;
mod config;
mod google_oauth;
mod handler;
mod model;
mod response;
mod db;
mod schema;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;

use model::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }
    dotenv().ok();
    env_logger::init();

    let app_state = AppState::init().await;
    let app_data = web::Data::new(app_state);
    let public_dir = std::env::current_dir().unwrap().join("public");

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&app_data.clone().env.client_origin)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(app_data.clone())
            .service(actix_files::Files::new("/api/images", &public_dir))
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    // .bind(("127.0.0.1", 8000))?
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

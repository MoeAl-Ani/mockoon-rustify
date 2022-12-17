mod model;
mod resources;
mod util;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use env_logger::Env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(resources::config)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
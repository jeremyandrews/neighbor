/// Won't you be my neighbor?
///
/// A simple CRM, starting as a JSON API user management system.
mod sitter;

use std::env;

use actix_web::{middleware, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use log::info;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await?;

    let listener = "127.0.0.1:5335";

    let server = HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // enable page served by header
            .wrap(middleware::DefaultHeaders::new().header("X-Neighbor", "0"))
            // pass database pool to application
            .data(db_pool.clone())
            // pass registered hooks to application
            .data(sitter::register_hooks())
            // @TODO: make endpoint paths easily configurable
            .configure(sitter::routes)
    });

    info!("Neighbor listening at: {:?}", listener);

    server.bind(listener)?.run().await?;

    Ok(())
}

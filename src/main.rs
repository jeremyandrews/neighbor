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
            // pass database pool to application so we can access it inside handlers
            .data(db_pool.clone())
            // @TODO: make endpoint paths easily configurable
            .configure(sitter::routes)
    });

    info!("Neighbor listening at: {:?}", listener);

    server
        .bind(listener)?
        .run()
        .await?;

    Ok(())
}

mod db;

use std::io;

use actix_web::{middleware, web, App, HttpRequest, HttpServer};
use sitter::person;

/// POC: list all persons defined in the Sitter database.
/// @TODO: optionally allow filtering to limit to 1 or more persons.
async fn read_person(_req: HttpRequest) -> String {
    let db = db::connect().await.unwrap();

    let persons = person::read(&db, None).await;
    format!("persons: {:#?}", persons)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // @TODO: Make server configurable.
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let listener = "127.0.0.1:5335";
    println!("Neighbor listening at: {:?}", listener);

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // expose API person endpoint
            // @TODO: make endpoint paths easily configurable
            .service(web::resource("/api/person").to(read_person))
    })
    .bind(listener)?
    .run()
    .await
}

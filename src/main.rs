mod db;

use std::io;

use log::info;

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use sitter::person;

/// POC: add a new person to the Person table.
async fn create_person(new_person: web::Json<person::NewPerson>) -> HttpResponse {
    let db = db::connect().await.unwrap();

    // @TODO: error handling
    let new_person_id = person::create(&db, &new_person.name, &new_person.email, &new_person.pass)
        .await
        .unwrap();
    info!(
        "created new person: {:#?} with id: {}",
        new_person, new_person_id
    );

    // Return the new person's id.
    HttpResponse::Ok().json(new_person_id)
}

/// POC: list all persons defined in the Person table.
/// @TODO: optionally allow filtering to limit to 1 or more persons.
async fn read_person(_req: HttpRequest) -> HttpResponse {
    let db = db::connect().await.unwrap();

    let persons = person::read(&db, None).await.unwrap();
    HttpResponse::Ok().json(persons)
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
            .service(web::resource("/api/person/create").route(web::post().to(create_person)))
    })
    .bind(listener)?
    .run()
    .await
}

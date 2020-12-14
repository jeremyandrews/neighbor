use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;

use sitter::person::{Person, PersonRequest};

#[get("/api/person")]
async fn read(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Person::read(None, db_pool.get_ref()).await;
    match result {
        Ok(person) => HttpResponse::Ok().json(person),
        _ => HttpResponse::BadRequest().body("Error trying to read persons from database")
    }
}

#[post("/api/person")]
async fn create(person: web::Json<PersonRequest>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Person::create(person.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(person) => HttpResponse::Ok().json(person),
        _ => HttpResponse::BadRequest().body("Error trying to create new person")
    }
}

// function that will be called on new Application to configure routes for this module
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(read);
    cfg.service(create);
}
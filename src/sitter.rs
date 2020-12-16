/// Exposes the Sitter API routes to Neighbor. Defines a Person
/// CRUD API, including: list, create, read, update and delete.
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use sitter::person::{Person, PersonHooks, PersonRequest};

/// Register Sitter's Password, Email, and Logger hook handlers.
/// @TODO: Make it possible to override with custom hook handlers.
pub fn register_hooks() -> PersonHooks {
    let mut person_hooks = PersonHooks::initialize();
    person_hooks.register_hook(sitter::password::Password);
    person_hooks.register_hook(sitter::email::Email);
    person_hooks.register_hook(sitter::logger::Logger);
    person_hooks
}

/// Configure Sitter API endpoints.
/// @TODO: Make it possible to define custom paths and/or endpoints.
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(create);
    cfg.service(update);
    cfg.service(read);
    cfg.service(delete);
}

/// List all Persons.
#[get("/api/person")]
async fn list(db_pool: web::Data<PgPool>, person_hooks: web::Data<PersonHooks>) -> impl Responder {
    let result = Person::read(None, db_pool.get_ref(), person_hooks.get_ref()).await;
    match result {
        Ok(person) => HttpResponse::Ok().json(person),
        _ => HttpResponse::BadRequest().body("Error trying to read persons from database"),
    }
}

/// Load a single Person.
#[get("/api/person/{uuid}")]
async fn read(
    uuid: web::Path<Uuid>,
    db_pool: web::Data<PgPool>,
    person_hooks: web::Data<PersonHooks>,
) -> impl Responder {
    let result = Person::read(
        Some(uuid.into_inner()),
        db_pool.get_ref(),
        person_hooks.get_ref(),
    )
    .await;
    match result {
        Ok(person) => HttpResponse::Ok().json(person),
        _ => HttpResponse::BadRequest().body("Error trying to read person from database"),
    }
}

/// Create a new Person.
#[post("/api/person")]
async fn create(
    person: web::Json<PersonRequest>,
    db_pool: web::Data<PgPool>,
    person_hooks: web::Data<PersonHooks>,
) -> impl Responder {
    let result = Person::create(
        person.into_inner(),
        db_pool.get_ref(),
        person_hooks.get_ref(),
    )
    .await;
    match result {
        Ok(person) => HttpResponse::Ok().json(person),
        Err(e) => {
            HttpResponse::BadRequest().body(format!("Error trying to create new person: {}", e))
        }
    }
}

/// Update an existing Person.
#[put("/api/person/{uuid}")]
async fn update(
    uuid: web::Path<Uuid>,
    person: web::Json<PersonRequest>,
    db_pool: web::Data<PgPool>,
    person_hooks: web::Data<PersonHooks>,
) -> impl Responder {
    let result = Person::update(
        uuid.into_inner(),
        person.into_inner(),
        db_pool.get_ref(),
        person_hooks.get_ref(),
    )
    .await;
    match result {
        Ok(person) => HttpResponse::Ok().json(person),
        Err(e) => HttpResponse::BadRequest().body(format!("Error updating Person: {}", e)),
    }
}

/// Delete a Person.
#[delete("/api/person/{uuid}")]
async fn delete(
    uuid: web::Path<Uuid>,
    db_pool: web::Data<PgPool>,
    person_hooks: web::Data<PersonHooks>,
) -> impl Responder {
    let result = Person::delete(uuid.into_inner(), db_pool.get_ref(), person_hooks.get_ref()).await;
    match result {
        Ok(rows) => {
            if rows > 0 {
                HttpResponse::Ok().body(format!("Successfully deleted {} person(s)", rows))
            } else {
                HttpResponse::BadRequest().body("Person not found")
            }
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Error deleting Person: {}", e)),
    }
}

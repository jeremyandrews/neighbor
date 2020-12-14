# Neighbor

Won't you be my neighbor?

A simple CRM, starting as a JSON API user management system.

## Installation

Neighbor uses the SQLx ORM to manage connections to a Postgres database. The CLI interface to SQLx must be used to install the schema.

First, install the CLI:

```sh
cargo install --version=0.2.0 sqlx-cli --no-default-features --features postgres
```

Next, create the Neighbor database:
```sh
echo DATABASE_URL=postgres://USERNAME:PASSWORD@localhost/neighbor > .env
sqlx database create
sqlx migrate run
```

## About

Neighbor is still in very early development. It is inspired by the CMS functionality found in Drupal 6, and aspires to eventually become a flexbile Content Management System in its own right.

The initial goal is to provide the functionality documented in this two-part series where we used Django, resulting in a Rust-powered JSON backend for a single page application:
https://www.tag1consulting.com/blog/building-api-django-20-part-i

We are currently building a self-hosted API powered by Actix and Tokio. We also intend to explore building the same self-hosted functionality using Tide and async_std.

## API

### Person

#### Create

Path: /api/person
Method: POST
Data: sitter::PersonRequest

POST a json-encoded sitter::NewPerson to create a new person. On success returns a json-encoded string containing the uuid of the newly created person.

Example:
```sh
curl -X POST -H "Content-Type: application/json" -d '{"name":"Somebody","email":"somebody@example.com","pass":"f00B@r"}' http://localhost:5335/api/person/create
"1f3bb010-6d1c-4439-a748-8ddb3ce7123d"
```

#### Read 

Path: /api/person
Method: GET
Data: Vec<sitter::Person>

Example:
```sh
curl http://localhost:5335/api/person
[{"id":"1f3bb010-6d1c-4439-a748-8ddb3ce7123d","name":"Somebody","email":"somebody@example.com","pass":"f00B@r"}]
```

Make an empty GET request to receive a json-encoded list of all sitter::Person objects in the database.

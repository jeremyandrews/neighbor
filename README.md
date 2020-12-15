# Neighbor

Won't you be my neighbor?

A simple CMS, starting as a JSON API user management system.

## Installation

Neighbor uses the SQLx ORM to manage connections to a Postgres database. The CLI interface to SQLx must be used to install the schema.

First, install the CLI:

```sh
cargo install --version=0.2.0 sqlx-cli --no-default-features --features postgres
```

Next, copy `example.env` to `.env` and edit the resulting file to match your local configuration:
```sh
cp example.env .env
```

Finally, create the Neighbor database:
```sh
sqlx database create
sqlx migrate run
```

## Usage

Use Cargo to run Neighbor, for example:

```sh
cargo run
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
Request-type: sitter::person::PersonRequest
Response-type: sitter::person::Person

POST a json-encoded sitter::person::PersonRequest to create a new Person. On success returns the newly created json-encoded Person, including the assigned uuid.

##### Example:
```sh
curl -X POST -H "Content-Type: application/json" -d '{"name":"Some Body","email":"somebody@example.com","pass":"Po(iUhJihU3$xS"}' http://localhost:5335/api/person
{"id":"5d62b617-67b6-4a3d-a2f1-f392f0ed64fd","name":"Some Body","email":"somebody@example.com","pass":"$argon2id$v=19$m=32768,t=1,p=4$KPjETcw8yJXhhTXqkKzj683/WYv5Av80$iBq4KS27a+C0SafTx2eSZQ"}
curl -X POST -H "Content-Type: application/json" -d '{"name":"Somebody Else","email":"somebodyelse@example.com","pass":"123456abcdef"}' http://localhost:5335/api/person
{"id":"1d66f0f4-88e8-4454-bd7e-445624bfd994","name":"Somebody Else","email":"somebodyelse@example.com","pass":"$argon2id$v=19$m=32768,t=1,p=4$KoH+adS/iJWO/mK7XzKHWZ4YaJOJCfnP$+vPFthc+/wnKHIJ2dktkWw"}
```

### Update

Path: /api/person/{id}
Method: PUT
Request-type: sitter::person::PersonRequest
Response-type: sitter::person::Person

PUT a json-encoded sitter::person::PersonRequest to update an existing Person, controlling which Person should be updated by specifying their uuid in the path. On success returns a json-encoded the updated Person.

##### Example:
```sh
curl -X PUT -H "Content-Type: application/json" -d '{"name":"Somebody","email":"somebody@example.com","pass":""}' http://localhost:5335/api/person/5d62b617-67b6-4a3d-a2f1-f392f0ed64fd
{"id":"5d62b617-67b6-4a3d-a2f1-f392f0ed64fd","name":"Somebody","email":"somebody@example.com","pass":"$argon2id$v=19$m=32768,t=1,p=4$KPjETcw8yJXhhTXqkKzj683/WYv5Av80$iBq4KS27a+C0SafTx2eSZQ"}
```

Note: @TODO The intent is that when an empty "pass" is set no change is made to the password, and when "pass" is not empty it can be used to change the password. Currently "pass" is completely ignored.

#### List 

Path: /api/person
Method: GET
Request-type: none
Response-type: Vec<sitter::person::Person>

Make an empty GET request to receive a json-encoded list of all sitter::Person objects.

@TODO Pager.

##### Example:
```sh
curl http://localhost:5335/api/person
[{"id":"1d66f0f4-88e8-4454-bd7e-445624bfd994","name":"Somebody Else","email":"somebodyelse@example.com","pass":"$argon2id$v=19$m=32768,t=1,p=4$KoH+adS/iJWO/mK7XzKHWZ4YaJOJCfnP$+vPFthc+/wnKHIJ2dktkWw"},{"id":"5d62b617-67b6-4a3d-a2f1-f392f0ed64fd","name":"Somebody","email":"somebody@example.com","pass":"$argon2id$v=19$m=32768,t=1,p=4$KPjETcw8yJXhhTXqkKzj683/WYv5Av80$iBq4KS27a+C0SafTx2eSZQ"}]
```

### Read

Path: /api/person/{id}
Method: GET
Request-type: none
Response-type: Vec<sitter::person::Person>

Make an empty GET request including a specific Uuid to receive a json-encoded list of the matching sitter::Person object.

##### Example:
```sh
curl http://localhost:5335/api/person/5d62b617-67b6-4a3d-a2f1-f392f0ed64fd
[{"id":"5d62b617-67b6-4a3d-a2f1-f392f0ed64fd","name":"Somebody","email":"somebody@example.com","pass":"$argon2id$v=19$m=32768,t=1,p=4$KPjETcw8yJXhhTXqkKzj683/WYv5Av80$iBq4KS27a+C0SafTx2eSZQ"}]
```

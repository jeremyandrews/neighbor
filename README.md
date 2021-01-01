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

To start over with a clean database, you can drop the old one and recreate:
```sh
sqlx database drop
sqlx database create
sqlx migrate run
```

## Usage

Use Cargo to run Neighbor, for example:

```sh
cargo run --release
```

## About

Neighbor is still in very early development. It is inspired by the CMS functionality found in Drupal 6, and aspires to eventually become a flexbile Content Management System in its own right.

The initial goal is to provide the functionality documented in this two-part series where we used Django, resulting in a Rust-powered JSON backend for a single page application:
https://www.tag1consulting.com/blog/building-api-django-20-part-i

Neighbor is currently a self-hosted API powered by Actix and Tokio with a PostgreSQL data store. The eventual goal is to support other frameworks (such as Tide and async_std) and other data stores (such as MySQL).

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
curl -X POST -H "Content-Type: application/json" -d '{"email":"somebody@example.com","pass":"Po(iUhJihU3$xS"}' http://localhost:5335/api/person
{"id":"80875435-1d6b-46fc-9515-255395f1925a","email":"somebody@example.com","pass":"$argon2id$v=19$m=262144,t=8,p=4$TcBXCrCUjnfIAcM2UNdtgzmy0ZiI2aYr$/GbooiDO/V3tIkYkmKrEeekGFv8CQXbyntF3wLeD0mI"}
curl -X POST -H "Content-Type: application/json" -d '{"email":"somebodyelse@example.com","pass":"123456abcdef"}' http://localhost:5335/api/person
{"id":"4899aecd-604a-4c7e-b265-1eeb4113d3bc","email":"somebodyelse@example.com","pass":"$argon2id$v=19$m=262144,t=8,p=4$V8tEvwgrhA2CHsfgxAbopZHPnX5eKqAx$UV3Bz5lQr0G0YMSdk0iH894cJqVEq+74mMVbzkTjiE8"}
```

### Update

Path: /api/person/{id}
Method: PUT
Request-type: sitter::person::PersonRequest
Response-type: sitter::person::Person

PUT a json-encoded sitter::person::PersonRequest to update an existing Person, controlling which Person should be updated by specifying their uuid in the path. On success returns a json-encoded the updated Person.

##### Example:
```sh
curl -X PUT -H "Content-Type: application/json" -d '{"email":"somebodyFOO@example.com","pass":""}' http://localhost:5335/api/person/80875435-1d6b-46fc-9515-255395f1925a
{"id":"80875435-1d6b-46fc-9515-255395f1925a","email":"somebodyFOO@example.com","pass":"$argon2id$v=19$m=262144,t=8,p=4$TcBXCrCUjnfIAcM2UNdtgzmy0ZiI2aYr$/GbooiDO/V3tIkYkmKrEeekGFv8CQXbyntF3wLeD0mI"}
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
[{"id":"4899aecd-604a-4c7e-b265-1eeb4113d3bc","email":"somebodyelse@example.com","pass":"$argon2id$v=19$m=262144,t=8,p=4$V8tEvwgrhA2CHsfgxAbopZHPnX5eKqAx$UV3Bz5lQr0G0YMSdk0iH894cJqVEq+74mMVbzkTjiE8"},{"id":"80875435-1d6b-46fc-9515-255395f1925a","email":"somebodyFOO@example.com","pass":"$argon2id$v=19$m=262144,t=8,p=4$TcBXCrCUjnfIAcM2UNdtgzmy0ZiI2aYr$/GbooiDO/V3tIkYkmKrEeekGFv8CQXbyntF3wLeD0mI"}]
```

### Read

Path: /api/person/{id}
Method: GET
Request-type: none
Response-type: Vec<sitter::person::Person>

Make an empty GET request including a specific Uuid to receive a json-encoded list of the matching sitter::Person object.

##### Example:
```sh
curl http://localhost:5335/api/person/80875435-1d6b-46fc-9515-255395f1925a
[{"id":"80875435-1d6b-46fc-9515-255395f1925a","email":"somebodyFOO@example.com","pass":"$argon2id$v=19$m=262144,t=8,p=4$TcBXCrCUjnfIAcM2UNdtgzmy0ZiI2aYr$/GbooiDO/V3tIkYkmKrEeekGFv8CQXbyntF3wLeD0mI"}]
```

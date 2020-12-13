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


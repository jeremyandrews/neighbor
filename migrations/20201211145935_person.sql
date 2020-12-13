-- Create Sitter table for storing persons.
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS person (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR (64) UNIQUE NOT NULL,
    email VARCHAR (254) UNIQUE NOT NULL,
    pass TEXT NOT NULL
)

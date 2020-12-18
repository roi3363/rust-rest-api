-- Your SQL goes here
create table people
(
    id         SERIAL PRIMARY KEY,
    first_name text NOT NULL,
    last_name  text NOT NULL,
    age        INT DEFAULT NULL,
    city       text DEFAULT NULL,
    country    text DEFAULT NULL,
    created_at    timestamp NOT NULL DEFAULT now(),
    updated_at    timestamp NOT NULL DEFAULT now()
)
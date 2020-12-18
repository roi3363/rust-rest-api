# Restful API example with Rust-Postgres-Rocket-Diesel
Just another API example with this classic combination

## Run
Make sure you have:
* Docker
* Cargo
* Diesel CLI

1. Run `docker run --name mydb -e POSTGRES_PASSWORD=123 -p 5432:5432 -d postgres`
2. `diesel run migrations` 
3. `cargo run` 

The server should be up and running on localhost:8000 
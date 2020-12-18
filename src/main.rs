#![feature(proc_macro_hygiene, decl_macro)]

pub mod models;
pub mod db;
pub mod schema;

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

use serde::{Serialize};
use rocket_contrib::json::Json;
use crate::models::{Person, UpdatePerson, NewPerson, get_people, delete_person, get_person};

#[derive(Debug, Serialize)]
struct ResponeMessage<T> {
    status_code: i32,
    message: String,
    content: T,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/people")]
fn people() -> Json<ResponeMessage<Vec<Person>>> {
    let people: Vec<Person> = get_people();
    let message = ResponeMessage {
        status_code: 200,
        message: String::from("OK"),
        content: people,
    };
    Json(message)
}

#[get("/person/<person_id>")]
fn person(person_id: i32) -> Json<ResponeMessage<Option<Person>>> {
    let person = get_person(person_id);
    let message;
    let status_code;
    let content;
    match person.is_some() {
        true => {
            message = String::from("OK");
            status_code = 200;
            content = Some(person.unwrap());
        },
        false => {
            message = String::from("Person could not be found");
            status_code = 400;
            content = None;
        },
    };
    Json(ResponeMessage {
        status_code,
        message,
        content
    })
}

#[post("/new-person", format = "json", data = "<new_person>")]
fn post(new_person: Json<NewPerson>) -> Json<ResponeMessage<usize>> {
    let num_of_inserts = new_person.create_person();
    let message = ResponeMessage {
        status_code: 201,
        message: String::from("OK"),
        content: num_of_inserts,
    };
    Json(message)
}

#[put("/update-person/<person_id>", format = "json", data = "<update_person>")]
fn put(update_person: Json<UpdatePerson>, person_id: i32) -> Json<ResponeMessage<usize>> {
    let num_of_updates = update_person.clone().update_person(person_id);
    let message = ResponeMessage {
        status_code: 201,
        message: String::from("OK"),
        content: num_of_updates
    };
    Json(message)
}

#[delete("/delete-person/<person_id>")]
fn delete(person_id: i32) -> Json<ResponeMessage<usize>> {
    let num_of_deletes = delete_person(person_id);
    let message = ResponeMessage {
        status_code: 201,
        message: String::from("OK"),
        content: num_of_deletes
    };
    Json(message)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, people, person, post, put, delete])
        .launch();
}

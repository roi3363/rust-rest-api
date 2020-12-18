use crate::schema::people;
use diesel::{RunQueryDsl, ExpressionMethods};
use crate::schema::people::columns;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::db::establish_conn;
use std::time::SystemTime;

#[table_name = "people"]
#[derive(Debug, Clone, AsChangeset, Serialize, Queryable)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: Option<i32>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[table_name = "people"]
#[derive(Debug, AsChangeset, Insertable, Deserialize)]
pub struct NewPerson {
    pub first_name: String,
    pub last_name: String,
    pub age: Option<i32>,
    pub city: Option<String>,
    pub country: Option<String>,
}

#[table_name = "people"]
#[derive(Debug, Clone, AsChangeset, Insertable, Deserialize)]
pub struct UpdatePerson {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub age: Option<i32>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub updated_at: Option<SystemTime>
}

pub fn get_person(person_id: i32) -> Option<Person> {
    let conn = establish_conn();
    let results = people::table
        .filter(columns::id.eq(person_id))
        .load::<Person>(&conn)
        .expect("Error loading person");
    match results.len() {
        0 => None,
        _ => Some(results[0].clone())
    }
}

pub fn get_people() -> Vec<Person> {
    let conn = establish_conn();
    let results = people::table
        .load::<Person>(&conn)
        .expect("Error loading data");
    results
}

pub fn delete_person(person_id: i32) -> usize {
    let conn = establish_conn();
    diesel::delete(people::table.filter(columns::id.eq(person_id)))
        .execute(&conn)
        .expect("Error delete person")
}

impl NewPerson {
    pub fn create_person(&self) -> usize {
        let conn = establish_conn();
        diesel::insert_into(people::table)
            .values(self)
            .execute(&conn)
            .expect("Error inserting person")
    }
}

impl UpdatePerson {
    pub fn update_person(self, person_id: i32) -> usize {
        let mut update_person = self.clone();
        update_person.updated_at = Some(SystemTime::now());
        let conn = establish_conn();
        diesel::update(people::table.filter(columns::id.eq(person_id)))
            .set(update_person)
            .execute(&conn)
            .expect(&format!("Unable to find person {:?}", person_id))
    }
}

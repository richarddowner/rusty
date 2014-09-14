extern crate postgres;
extern crate time;
extern crate serialize;
extern crate nickel;

use std::io::net::ip::Ipv4Addr;
use serialize::json;
use postgres::{PostgresConnection, NoSsl};
use nickel::{Nickel, Request, Response};

use models::{ Practice, Person };
use database::{ Database };

mod models;
mod database;

#[deriving(Decodable, Encodable)]
pub struct PersonForm {
    pub name: String,
    pub display_name: String,
    pub age: Option<u8>,
    pub data: Option<Vec<u8>>,
}

fn main() {    

    // create database
    let database = Database::new();
    database.create();

    // insert a dummy person into the database
    Person::seed_database();
    Person::query_example();

    Practice::seed_database();

    // create a new web server
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    // routes
    router.get("/", home_handler);
    router.post("/person", create_person_handler);

    // middleware
    server.utilize(Nickel::json_body_parser());
    server.utilize(router);

    // start server
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}

fn home_handler (_request: &Request, response: &mut Response) {
    response.send("hai");
}

fn create_person_handler (request: &Request, response: &mut Response) {
    let person = request.json_as::<PersonForm>().unwrap();

    let numbers = match person.data {
        Some(x) => format!("Got some numbers: {}", x).to_string(),
        None => "None".to_string()
    };

    let age = match person.age {
        Some(x) => if x >= 30 { "You are old!".to_string() } else { "You are young!".to_string() },
        None => "".to_string()
    };

    let text = format!("name: {}, display name: {}, age: {}, numbers: {}", person.name, person.display_name, age, numbers);
    
    response.send(text.as_slice());
}
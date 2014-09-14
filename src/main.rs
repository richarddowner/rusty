extern crate postgres;
extern crate time;
extern crate serialize;
extern crate nickel;

use std::io::net::ip::Ipv4Addr;
use time::Timespec;
use serialize::json;
use serialize::json::Json;
use postgres::{PostgresConnection, NoSsl};
use postgres::types::ToSql;
use nickel::{Nickel, Request, Response};

struct Person {
    id: i32,
    name: String,
    created: Timespec,
    data: Option<Vec<u8>>,
    json: Option<Json>,
}

#[deriving(Decodable, Encodable)]
struct PersonForm {
    name: String,
    display_name: String,
    age: Option<u8>,
    data: Option<Vec<u8>>,
}

fn main() {

    // connect to postgres
    let conn = PostgresConnection::connect("postgres://postgres:swine@localhost/rusty", &NoSsl).unwrap();
    
    // ddl statements
    conn.execute("DROP TABLE IF EXISTS person;", []).unwrap();
    
    conn.execute(       
        "CREATE TABLE IF NOT EXISTS person (
            id          SERIAL PRIMARY KEY,
            name        VARCHAR NOT NULL,
            created     TIMESTAMP NOT NULL,
            data        BYTEA,
            json        JSON
        );", []).unwrap();

    
    
    // insert example
    println!("inserting");

    let person = Person {
        id: 0,
        name: "Jake Scott".to_string(),
        created: time::get_time(),
        data: Some(vec!(1, 2, 3)),
        json: Some(json::from_str(r#"{"city": "London", "lat": 51.507222, "lon": -0.1275}"#).unwrap()),
    };

    conn.execute(
        "INSERT INTO person (
            name, 
            created, 
            data, 
            json
        ) VALUES ($1, $2, $3, $4);", 
        &[
            &person.name, 
            &person.created, 
            &person.data,
            &person.json,
        ]
    ).unwrap();


    // create another person
    let person2 = Person {
        id: 0,
        name: "John John Florence".to_string(),
        created: time::get_time(),
        data: Some(vec!(1, 2, 3)),
        json: Some(json::from_str(r#"{"city": "Hawaii", "lat": 21.3114, "lon": 157.7964}"#).unwrap()),
    };    


    // transaction example
    println!("inserting using transaction");

    let trans = conn.transaction().unwrap();
    trans.execute(
        "INSERT INTO person (
            name, 
            created, 
            data, 
            json
        ) VALUES ($1, $2, $3, $4);", 
        &[
            &person2.name, 
            &person2.created, 
            &person2.data,
            &person2.json,
        ]
    ).unwrap();    
    trans.set_commit();
    trans.finish().unwrap();

    
    // query
    println!("querying");

    let stmt = conn.prepare("SELECT id, name, created, data, json FROM person;").unwrap();

    for row in stmt.query([]).unwrap() {
        
        let p = Person {
            id: row.get(0u),
            name: row.get(1u),
            created: row.get(2u),
            data: row.get(3u),
            json: row.get(4u),
        };

        println!("Found person {}", p.name);
    }

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
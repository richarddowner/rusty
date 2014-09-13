extern crate postgres;
extern crate time;
extern crate serialize;

use time::Timespec;
use serialize::json;
use serialize::json::Json;

use postgres::{PostgresConnection, NoSsl};
use postgres::types::ToSql;

struct Person {
    id: i32,
    name: String,
    created: Timespec,
    data: Option<Vec<u8>>,
    json: Option<Json>,
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
    println("inserting using transaction");

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
}
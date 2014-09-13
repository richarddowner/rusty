extern crate postgres;
extern crate time;

use time::Timespec;

use postgres::{PostgresConnection, NoSsl};
use postgres::types::ToSql;

struct Person {
    id: i32,
    name: String,
    created: Timespec,
    data: Option<Vec<u8>>
}

fn main() {
    let conn = PostgresConnection::connect("postgres://postgres:swine@localhost/rusty", &NoSsl).unwrap();
    
    conn.execute("DROP TABLE IF EXISTS person;", []).unwrap();

    conn.execute(       
        "CREATE TABLE IF NOT EXISTS person (
            id          SERIAL PRIMARY KEY,
            name        VARCHAR NOT NULL,
            created     TIMESTAMP NOT NULL,
            data        BYTEA
        );", []).unwrap();

    let person = Person {
        id: 0,
        name: "Jake Scott".to_string(),
        created: time::get_time(),
        data: None
    };

    let trans = conn.transaction().unwrap();
    
    trans.execute("INSERT INTO person (name, created, data) VALUES ($1, $2, $3);", 
        &[&person.name, &person.created, &person.data]).unwrap();
    
    trans.set_commit();
    
    trans.finish().unwrap();

    let stmt = conn.prepare("SELECT id, name, created, data FROM person;").unwrap();

    for row in stmt.query([]).unwrap() {
        
        let p = Person {
            id: row.get(0u),
            name: row.get(1u),
            created: row.get(2u),
            data: row.get(3u)
        };

        println!("Found person {}", p.name);
    }
}
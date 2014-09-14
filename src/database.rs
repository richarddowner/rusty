extern crate postgres;

use postgres::{PostgresConnection, NoSsl};

pub struct Database;

impl Database {

    pub fn new() -> Database {
        Database
    }

    pub fn connect (&self) -> PostgresConnection {
        PostgresConnection::connect("postgres://postgres:swine@localhost/rusty", &NoSsl).unwrap()
    }

    pub fn create (&self) {
        let conn = self.connect();
        conn.execute("DROP TABLE IF EXISTS practice;", []).unwrap();        
        conn.execute(       
            "CREATE TABLE IF NOT EXISTS practice (
                id                  SERIAL PRIMARY KEY,
                name                VARCHAR NOT NULL,
                display_name        VARCHAR,
                logo_document_id    VARCHAR,
                avatar_document_id  VARCHAR
            );", []).unwrap();
    }   
}
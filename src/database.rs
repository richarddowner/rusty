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
        
        conn.execute("DROP TABLE IF EXISTS person;", []).unwrap();
        
        conn.execute(       
            "CREATE TABLE IF NOT EXISTS person (
                id          SERIAL PRIMARY KEY,
                name        VARCHAR NOT NULL,
                created     TIMESTAMP NOT NULL,
                data        BYTEA,
                json        JSON
            );", []).unwrap();

        conn.execute("DROP TABLE IF EXISTS practice;", []).unwrap();
        
        conn.execute(       
            "CREATE TABLE IF NOT EXISTS practice (
                id                  SERIAL PRIMARY KEY,
                name                VARCHAR NOT NULL,
                display_name        VARCHAR NOT NULL,
                logo_document_id    VARCHAR NOT NULL,
                avatar_document_id  VARCHAR NOT NULL                
            );", []).unwrap();
    }   
}
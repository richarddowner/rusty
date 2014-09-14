extern crate serialize;
extern crate time;

use time::Timespec;

use serialize::json;
use serialize::json::Json;

use database::{ Database };

pub struct Person {
    pub id: i32,
    pub name: String,
    pub created: Timespec,
    pub data: Option<Vec<u8>>,
    pub json: Option<Json>,
}

impl Person {
    
    pub fn seed_database () {

        let person = Person {
            id: 0,
            name: "Jake Scott".to_string(),
            created: time::get_time(),
            data: Some(vec!(1, 2, 3)),
            json: Some(json::from_str(r#"{"city": "London", "lat": 51.507222, "lon": -0.1275}"#).unwrap()),
        };

        let conn = Database::new().connect();

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
        
    }

    pub fn query_example () {
        let conn = Database::new().connect();
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
}


pub struct Practice {
    pub id: i32,
    pub name: String,
    pub display_name: String,
    pub logo_document_id: String,
    pub avatar_document_id: String,    
}

impl Practice {
    pub fn seed_database () {

        let practice = Practice {
            id: 0,
            name: "Pacific Bay Associates (Ltd)".to_string(),
            display_name: "Pacific Bay Associates".to_string(),
            logo_document_id: "01234567-89ab-cdef-0123-456789abcdef".to_string(),
            avatar_document_id: "01234567-89ab-cdef-0123-456789abcdef".to_string(),         
        };

        let conn = Database::new().connect();

        conn.execute(
            "INSERT INTO practice (
                name, 
                display_name, 
                logo_document_id, 
                avatar_document_id
            ) VALUES ($1, $2, $3, $4);",
            &[
                &practice.name,
                &practice.display_name,
                &practice.logo_document_id,
                &practice.avatar_document_id,
            ]
        ).unwrap();     
    }
}
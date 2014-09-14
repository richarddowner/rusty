extern crate serialize;
extern crate time;

use time::Timespec;

use serialize::json;
use serialize::json::Json;

use database::{ Database };

#[deriving(Show)]
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

    pub fn query_example () {
        let conn = Database::new().connect();
        let stmt = conn.prepare("SELECT id, name, display_name, logo_document_id, avatar_document_id FROM practice;").unwrap();
        for row in stmt.query([]).unwrap() {            
            let p = Practice {
                id: row.get(0u),
                name: row.get(1u),
                display_name: row.get(2u),
                logo_document_id: row.get(3u),
                avatar_document_id: row.get(4u),                
            };
            println!("Found pracice {}", p);
        }
    }
}
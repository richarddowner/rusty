extern crate serialize;
extern crate time;

use time::Timespec;

use serialize::json;
use serialize::json::Json;

use database::{ Database };
use postgres::types::ToSql;

#[deriving(Show)]
#[deriving(Encodable, Decodable)]
pub struct Practice {
    pub id: i32,
    pub name: String,
    pub display_name: Option<String>,
    pub logo_document_id: Option<String>,
    pub avatar_document_id: Option<String>,
}

#[deriving(Show)]
#[deriving(Encodable, Decodable)]
pub struct PracticeForm {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub logo_document_id: Option<String>,
    pub avatar_document_id: Option<String>,
}

impl Practice {

    pub fn insert(practice: &mut Practice) {
        let conn = Database::new().connect();        
        let stmt = conn.prepare(
            "INSERT INTO practice (
                name, 
                display_name, 
                logo_document_id, 
                avatar_document_id
            ) 
            VALUES (
                $1, $2, $3, $4
            )
            RETURNING id;").unwrap();

        let params: &[&ToSql] = &[
            &practice.name,
            &practice.display_name,
            &practice.logo_document_id,
            &practice.avatar_document_id,
        ];

        for row in stmt.query(params).unwrap() {
            let id = row.get(0u);
            practice.id = id;
            break;
        }
    }

    pub fn seed_database () {
        let practice = Practice {
            id: 0,
            name: "Pacific Bay Associates (Ltd)".to_string(),
            display_name: Some("Pacific Bay Associates".to_string()),
            logo_document_id: Some("01234567-89ab-cdef-0123-456789abcdef".to_string()),
            avatar_document_id: Some("01234567-89ab-cdef-0123-456789abcdef".to_string()),            
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
extern crate lmdb;

use std::io::fs;
use std::path::Path;
use lmdb::base::{Environment, Database};

pub struct Cache {
    env: Environment,
    database: Database
}

impl Cache {

    pub fn new() -> Cache {
        let path = Path::new("./cache");

        let display = path.display();

        if path.exists() {
            fs::rmdir_recursive(&path);
        }

        let mut env = Environment::new().unwrap();
        
        env.set_maxdbs(5).unwrap();
        env.open(&path, 0, 0o755).unwrap();

        let database:Database = env.get_or_create_db("db", 0).unwrap();

        Cache {
            env: env,
            database: database
        }
    }

    pub fn set(&self, key: &str, value: &str) {
        let mut transaction = self.env.new_transaction().unwrap();
        transaction.set(&self.database, &key, &value).unwrap();
        transaction.commit();
    }

    pub fn get(&self, key: &str) -> String {
        let transaction = self.env.new_transaction().unwrap();
        let v: String = transaction.get(&self.database, &key).unwrap();
        v
    }
}
extern crate postgres;
extern crate time;
extern crate serialize;
extern crate nickel;
extern crate lmdb;

use std::io::fs;
use std::path::Path;

use std::io::net::ip::Ipv4Addr;
use serialize::json;
use postgres::{PostgresConnection, NoSsl};
use nickel::{Nickel, Request, Response};
use lmdb::base::{Environment};

use models::{ Practice };
use database::{ Database };

mod models;
mod database;
mod controllers;

fn main() {    

    let path = Path::new("./dbs");
    let display = path.display();
    if path.exists() {
        println!("{} exists", display);
        fs::rmdir_recursive(&path);
    }    
    let mut env = Environment::new().unwrap();
    env.set_maxdbs(5);
    env.open(&path, 0, 0o755);
    env.get_or_create_db("test-db", 0);

    println!("Lighting bolts!");


    // create database
    let database = Database::new();
    database.create();    

    Practice::seed_database();    

    // create a new web server
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    // routes
    router.get("/",             controllers::get_home);
    router.get("/healthcheck",  controllers::get_healthcheck);
    router.get("/practice",     controllers::get_practices);
    router.post("/practice",    controllers::post_practice);

    // middleware
    server.utilize(Nickel::json_body_parser());
    server.utilize(Nickel::query_string());
    server.utilize(router);

    // start server
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
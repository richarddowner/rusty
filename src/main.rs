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
use cache::{ Cache };

mod models;
mod database;
mod controllers;
mod cache;

fn main() {    

    // create lmdb based cache
    let cache: Cache = Cache::new();

    cache.set("/practice", r#" { "name" : "Superlogical", "display_name": "Bob" } "#);
    let value:String = cache.get("/practice");
    println!("got {}", value);

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
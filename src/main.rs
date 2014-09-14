extern crate postgres;
extern crate time;
extern crate serialize;
extern crate nickel;

use std::io::net::ip::Ipv4Addr;
use serialize::json;
use postgres::{PostgresConnection, NoSsl};
use nickel::{Nickel, Request, Response};

use models::{ Practice };
use database::{ Database };

mod models;
mod database;
mod controllers;

fn main() {    

    // create database
    let database = Database::new();
    database.create();    

    Practice::seed_database();
    Practice::query_example();

    // create a new web server
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    // routes
    router.get("/",             controllers::get_home);
    router.get("/healthcheck",  controllers::get_healthcheck);
    router.post("/practice",    controllers::post_practice);

    // middleware
    server.utilize(Nickel::json_body_parser());
    server.utilize(router);

    // start server
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
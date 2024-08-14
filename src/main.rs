mod common;
mod handler;

extern crate dotenv;

use crate::handler::get_all_handlers;

use common::db::connect_to_database;
use dotenv::dotenv;
use log::info;
use std::{
    env,
    net::{
        IpAddr,
        Ipv4Addr,
        SocketAddr,
    },
};

#[tokio::main]
async fn main() {
    // Read environment variables
    dotenv().ok();

    // Initialize logger
    colog::init();

    // Connect to database
    info!("Connecting to database...");
    let _db = connect_to_database();

    // Setup router
    let router = get_all_handlers();

    // Start the server
    let host_str = env::var("SERVER_HOST").expect("Missing environment variable SERVER_HOST");
    let port_str = env::var("SERVER_PORT").expect("Missing environment variable SERVER_PORT");

    let host: Vec<u8> = host_str
        .split(".")
        .map(|part| part.parse::<u8>().expect("Invalid host"))
        .collect();
    let port = port_str.parse::<u16>().expect("Invalid port");
    let address = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(host[0], host[1], host[2], host[3])),
        port,
    );
    info!("Starting Template API at address {}:{}", host_str, port_str);

    warp::serve(router).run(address).await;
}

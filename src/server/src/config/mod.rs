use dotenv::dotenv;
use std::net::{Ipv4Addr, SocketAddrV4};

pub mod db;

pub struct Settings {
    pub server_addr: SocketAddrV4,
    pub database_url: String,
}

impl Default for Settings {
    fn default() -> Self {
        dotenv().ok();

        let host = std::env::var("SERVER_HOST")
            .expect("SERVER_HOST must be set")
            .parse::<Ipv4Addr>()
            .expect("SERVER_HOST must be valid");

        let port = std::env::var("SERVER_PORT")
            .expect("SERVER_PORT must be set")
            .parse::<u16>()
            .expect("SERVER_PORT must be valid");

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self {
            server_addr: SocketAddrV4::new(host, port),
            database_url,
        }
    }
}

use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

/// Establish a database connection
pub fn connect_to_database() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
use std::{time::SystemTime};

use diesel::prelude::*;


pub fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::bashrc)]
pub struct Bashrc {
    id: i32,
    name: String,
    created_at: SystemTime,
    updated_at: SystemTime,
}
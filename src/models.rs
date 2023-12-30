use chrono::{DateTime, Utc, NaiveDateTime};
use diesel::prelude::*;

use crate::schema::bashrc;

#[derive(Insertable)]
#[diesel(table_name = bashrc)]
pub struct NewBashrc<'a> {
    pub name: &'a str,
    
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::bashrc)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Bashrc {
    pub id: i32,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

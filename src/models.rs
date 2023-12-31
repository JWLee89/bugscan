use chrono::{DateTime, Utc, NaiveDateTime};
use diesel::prelude::*;

use crate::schema::shell_commands;

#[derive(Insertable)]
#[diesel(table_name = shell_commands)]
pub struct NewShellCommand<'a> {
    pub name: &'a str,
    pub shell_command: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::shell_commands)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ShellCommand {
    pub id: i32,
    pub name: String,
    pub shell_command: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

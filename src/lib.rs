use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use schema::shell_commands::shell_command;
use std::env;
use crate::models::ShellCommand;

use self::models::NewShellCommand;
mod models; // Add the missing module declaration for `models`
mod fs;
mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

 /// Get the current git branch that we are currently on
 pub fn get_current_git_branch() -> String {
    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("failed to execute command: git rev-parse --abbrev-ref HEAD");

    let output = String::from_utf8_lossy(&output.stdout);
    let output = output.to_string();
    let output = output.split("\n").collect::<Vec<&str>>();
    let output = output[0].to_string();
    let output = output.replace("* ", "");
    output
}

pub fn create_shell_command(conn: &mut SqliteConnection, name: &str, command: &str) {
    use schema::shell_commands;

    let new_shell_command = NewShellCommand {
        name,
        shell_command: command
    };

    diesel::insert_into(shell_commands::table)
        .values(&new_shell_command)
        .execute(conn)
        .expect("Error saving new bashrc");
}

pub fn get_bashrc(conn: &mut SqliteConnection, name: &str) {
    use self::schema::shell_commands::dsl::shell_commands;

    let result = shell_commands
        .filter(schema::shell_commands::name.eq(name))
        .load::<ShellCommand>(conn)
        .expect("Error loading bashrc data from database");

    println!("Displaying {} bashrc", result.len());
}


pub mod arguments {
    use clap::{Subcommand};

    // This is a list of all the arguments that can be passed to the program
    #[derive(Subcommand, Debug)]
    pub enum Command {
        /// Add a new command to the bashrc file
        Command {
            #[clap(short='n', help="Name of the command")]
            add: String,
            #[clap(help="Show the commands")]
            ls: String,
        },
        /// List all the current settings
        List {
            #[clap(short, short='v')]
            verbose: bool,
        },
        /// Save the current state of the bashrc file
        /// -n: the name of the current setting
        /// E.g. save -n default
        Save {
            #[clap(short, long, short='n', 
                help="The name of the current setting. 
                E.g. \"save -n default\" // save current setting as \"default\"")]
            name: String,
        },
        Load,
        Version
    }
} 

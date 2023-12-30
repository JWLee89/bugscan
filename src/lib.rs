use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;


mod schema;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub mod arguments {
    use clap::{Subcommand};

    // This is a list of all the arguments that can be passed to the program
    // E.g. teemo --help
    // E.g. teemo --version
    // E.g. teemo --list
    #[derive(Subcommand, Debug)]
    pub enum Command {
        /// Set the path to the bashrc file
        Bind {
            #[clap(short='p', help="Set the path to the bashrc file")]
            path: String,
            #[clap(short='s', help="Show the currently bound path")]
            show: bool,
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

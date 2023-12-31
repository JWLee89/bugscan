use std::{fs::File, io::Read};

// TODO: update using clap
use clap::{Parser};
use bashrc_manager::{arguments::Command, establish_connection, get_current_git_branch, get_bashrc};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

pub struct BashrcFile<'a> {
    file_path: &'a str, 
}

impl<'a> BashrcFile<'a> {
    pub fn new(file_path: &'a str) -> Self {
        BashrcFile {
            file_path: file_path,
        }
    }

    pub fn read(&self) -> String {
        let mut file = File::open(self.file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    pub fn write(&self, file_path: &str) {
        let mut source = File::open(self.file_path).unwrap();
        let mut target = File::create(file_path).unwrap();
        std::io::copy(&mut source, &mut target).unwrap();
    }
}


fn main() {
    let args = Args::parse();
    let command_executed = &args.command;

    let mut connection = establish_connection();
    let fs = BashrcFile { 
        file_path: "",
    };
    // println!("Contents: {:?}", fs.read());
    let branch = get_current_git_branch();
    println!("Current branch is : {:?}", branch);
    // Insert into database
    // create_bashrc(&mut connection, "default");
    // get_bashrc(&mut connection, "default");

    match command_executed {
        Command::List { verbose } => {
            println!("List: {:?}", verbose);
        },
        // Save current command
        Command::Save { name } => {
            println!("Save: {:?}", name);
        },
        // If particular command is not implemented, we will panic.
        // The developer screwed up.
        _ => {
            panic!("Command: {:?} not implemented", command_executed);
        }
    };
}

// TODO: update using clap
use clap::{Parser};
use bashrc_manager::arguments::Command;
use bashrc_manager::database::database::Database;
use rusqlite::{Connection, Result as SqlResult};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}


fn main() {
    let args = Args::parse();
    let command_executed = &args.command;
    let database = Database::new("bashrc_manager.db");

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

    
    // Note need extra "--" when using "cargo run" command
    // E.g. "cargo run -- --verbose"
    // let matches = Command::new("Captain teemo").arg(
    //         Arg::new(UtilArguments::List)
    //             .short('v')
    //             .required(true)
    //             .long("verbose")
    //             .action(ArgAction::Count)
    //     )
    //     .get_matches();
    // println!("Verbose: {}", matches.get_count("verbose"));
}

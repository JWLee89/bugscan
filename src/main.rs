// TODO: update using clap
use clap::{Parser};
use bashrc_manager::arguments::Command;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}


fn main() {
    let args = Args::parse();
    dbg!(args);

    
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

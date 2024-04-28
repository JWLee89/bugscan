// Right now, I know that this codebase is not optimal.
// I am going to get the basic functionality working first.
// Afterwards, I plan on refactoring this codebase to learn more about rust
// and how things work.

use std::{process::Command as Cmd, char};
use std::collections::VecDeque;

// TODO: update using clap
use clap::{Parser};
use bashrc_manager::{arguments::Command, establish_connection, create_shell_command};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// A stored argument that is saved in the database.
/// id: The ID of the stored argument 
/// name: The name of the argument
/// The alias: Another alias to refer to the StoredArgument
/// value: The static value we plan on assigning to the stored 
/// argument
struct StoredArgument {
    id: u32,
    name: String,
    alias: Option<String>,
    value: String,
}

/// TODO: Rename this to CommandTemplate
/// A stored command that is saved in the database.
/// It is used to templatize stored commands such as to following
/// 
struct StoredCommand {
    name: String,
    arguments: Vec<StoredArgument>,
    command: String,
}

impl StoredCommand {
    fn new(name: String, arguments: Vec<StoredArgument>, command: String) -> Self {
        Self {
            name,
            arguments,
            command,
        }
    }
    /// This should transform the following message: 
    /// git commit -m \"[feat][$(git rev-parse --abbrev-ref HEAD)] {{message}}\"
    /// into:
    /// git commit -m \"[feat][feat-branch] I am a message\"
    /// note: $(git rev-parse --abbrev-ref HEAD) is evaluated here
    /// into "feat-branch"
    fn parse_subcommand(&self) {

    }
    // Create a command from a stored command. 
    // Stored commands can be retrieved from the database
    // fn get_command(&self) -> Command {
    //     let mut command = self.command.clone();
    //     let command_to_run = Command::new("echo");
    //     for argument in &self.arguments {
    //         Command:
    //         let argument_name = format!("{{{}}}", argument.name);
    //         command = command.replace(&argument_name, &argument.value);
    //     }
    //     command
    // }
}

fn handle_doubly_curly_braces(input: String) -> String {
    // A deque of two curly braces
    let mut open_stack: VecDeque<usize> = VecDeque::new();
    let mut close_stack: VecDeque<usize> = VecDeque::new();
    let mut chars = input.chars();
    // For now, assume that at least one character exists.
    let mut prev = chars.next().unwrap();
    
    // Opening and closing braces to monitor
    let opening = '{';
    let closing = '}';

    // Final output created via string concatenation
    let mut output = String::from("");
    let mut output_arr = Vec::new();

    // E.g. Should be able to capture nested double curly brackets
    // some str {{ hello {{ I am teemo }} cowbell }}
    // some str hello I am teemo cowbell
    let start_index = 0;
    for (index, current) in chars.enumerate() {
        // "}}"
        if prev == closing && current == closing {
            if open_stack.len() == 0 {
                panic!("No set of open brackets. Ignore");
            }
             // parse
            let open_index = open_stack.pop_back().unwrap();
            let substring: String = input.chars()
                .skip(open_index + 2)
                .take(index - open_index - 2).collect();
            println!("Close: {index}, open: {open_index}, substring: {substring}");
            output += &substring;
        } 
        // "{{"
        if prev == opening && current == opening {
            open_stack.push_back(index);
            let moo: String = input.chars()
                .skip(start_index)
                .take(index - start_index - 1)
                .collect();
            output += &moo;
        }
        prev = current;
        output_arr.push(current);
    }

    // Do error handling somewhere.
    if open_stack.len() != 0 || close_stack.len() != 0 {
        panic!("teemo")
    }
    println!("Output arr: {:?}", output_arr);
    output
}


/**
 * Behavior of the application
 * 1. Can store templated commands
 * 2. 
 */
fn main() {
    let args = Args::parse();
    // Get command that was passed by user
    let command_executed = &args.command;

    let current_command = StoredCommand {
        name: "commit-feat".to_string(),
        arguments: vec![
            StoredArgument {
                id: 2,
                name: "message".to_string(),
                alias: Some("m".to_string()),
                value: "captain teemo on duty".to_string(),
            },
        ],
        command: "git commit -m \"[feat][$(git rev-parse --abbrev-ref HEAD)] {{message}}\"".to_string(),
    };

    current_command.parse_subcommand();    

    // // let command_str = current_command.get_command();
    // // println!("Command: {command_str}");
    let cow = Cmd::new("rm").arg("-rf").arg("test_folder")
    .output()
    .expect("yee");
    let ls_output = String::from_utf8(cow.stdout).unwrap();
    
    let command_to_run = Cmd::new("echo")
        .arg(
            format!("Captain teemo: {}", ls_output)
        )
        .output().expect("failed to execute process")
        ;
    // Yee
    println!("Command: {:?}", std::str::from_utf8(&command_to_run.stdout));

    // TODO: Parse double curly braces
    // This should yield
    // Substring 1: new cow 
    // substring 2: {{ template new cow }}
    // Output: git commit -m template new cow
    let sample_input = "git commit -m {{ template {{ new cow}} }}".to_string();
    let output = handle_doubly_curly_braces(sample_input);
    println!("Double curly braces: {output}");


    // want to store the following
    // name: commit-feat (name of command)
    // arguments: "-m, message"
    // {{ }} == static. Always included
    // { } == dynamic. Passed in by user.
    // command: git commit -m "[feat][{{branch-name}}]{message}}"
    // E.g. use case: "commit-feat"

    // TODO: Initialize the application
    // let mut connection = establish_connection();

    // Insert into database
    // create_shell_command(&mut connection, "git feature", "captain teemo on duty");
    // get_bashrc(&mut connection, "default");

    match command_executed {
        Command::Command { add, ls } => {
            println!("Add: {:?}, List: {:?}", add, ls);
            if add.is_empty() {

            }
        },
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

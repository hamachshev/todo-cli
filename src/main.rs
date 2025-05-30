use clap::{Parser, Subcommand};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// A simple todo list application
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all todos
    List,
    /// Add a new todo item
    Add {
        /// Item to add to the todo list
        item: String,
    },
    /// Mark a done todo item
    Mark {
        /// Item to add to the todo list
        item: String,
    },
}

fn main() {
    let mut todos = load_todos();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { item }) => {
            println!("adding item: {}", item);
            todos.push(format!("- {}", item).to_string());
            save_todos(&todos);
        }
        Some(Commands::List) => {
            println!("Listing all todos:");
            for todo in &todos {
                println!("{}", todo);
            }
        }
        Some(Commands::Mark { item }) => {
            println!("marking item as done: {}", item);
        }
        None => {}
    }
}

fn load_todos() -> Vec<String> {
    let file_path = "todos.bin";
    if let Ok(mut file) = File::open(file_path) {
        println!("Loading existing todo list from {}", file_path);

        let todos_read: Vec<String> =
            bincode::decode_from_std_read(&mut file, bincode::config::standard()).expect(
                "Failed to deserialize todo list. Ensure the file is in the correct format.",
            );
        return todos_read;
    } else {
        println!("No existing todo list found, starting fresh.");
        return Vec::new();
    }
}

fn save_todos(todos: &[String]) -> () {
    let file_path = "todos.bin";
    let mut file = File::create(file_path).expect("Failed to open todo file for writing");
    // let mut writer = BufWriter::new(file);
    bincode::encode_into_std_write(todos, &mut file, bincode::config::standard())
        .expect("Failed to serialize todo list");
}

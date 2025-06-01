use clap::{Parser, Subcommand};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
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
        /// Number of the item to mark as done
        item: u16,
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
            println!("listing all todos:");
            list_todos(&todos);
        }
        Some(Commands::Mark { item }) => {
            println!("marking item as done: {}", item);
            let index = *item as usize - 1;
            todos[index] = format!("x {}", &todos[index][2..]);
            list_todos(&todos);
            save_todos(&todos);
        }
        None => {}
    }
}

fn list_todos(todos: &[String]) {
    for (index, todo) in todos.iter().enumerate() {
        println!("{}. {}", (index + 1), todo);
    }
}
fn load_todos() -> Vec<String> {
    let file_path = "todos.bin";
    if let Ok(file) = File::open(file_path) {
        println!("Loading existing todo list from {}", file_path);
        let mut reader = BufReader::new(file);
        let todos_read: Vec<String> =
            bincode::decode_from_std_read(&mut reader, bincode::config::standard()).unwrap_or_else(
                |_| {
                    println!("Failed to deserialize todo list. Starting fresh.");
                    Vec::new()
                },
            );
        return todos_read;
    } else {
        println!("No existing todo list found, starting fresh.");
        return Vec::new();
    }
}

fn save_todos(todos: &[String]) {
    let file_path = "todos.bin";
    let file = File::create(file_path).expect("Failed to open todo file for writing");
    let mut writer = BufWriter::new(file);
    bincode::encode_into_std_write(todos, &mut writer, bincode::config::standard())
        .expect("Failed to serialize todo list");
}

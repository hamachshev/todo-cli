use clap::{Parser, Subcommand};

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
    let mut todos = Vec::new();

    load_todos(&mut todos);

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { item }) => {
            println!("adding item: {}", item);
            todos.push(item.to_string());
        }
        Some(Commands::List) => {
            println!("Listing all todos:");
            for todo in &todos {
                println!("- {}", todo);
            }
        }
        Some(Commands::Mark { item }) => {
            println!("marking item as done: {}", item);
        }
        None => {}
    }
}

fn load_todos(todos: &[String]) -> _ {}

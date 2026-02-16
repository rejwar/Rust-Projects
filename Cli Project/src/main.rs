mod commands;
mod db;
mod models;

use clap::{Parser, Subcommand};
use models::Priority;

#[derive(Parser)]
#[command(name = "RustTodo")]
#[command(about = "A simple CLI to manage your daily tasks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        description: String,

        #[arg(short, long, default_value = "medium")]
        priority: String,
    },

    List,

    Done {
        id: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add {
            description,
            priority,
        } => {
            let p = match priority.to_lowercase().as_str() {
                "high" => Priority::High,
                "low" => Priority::Low,
                _ => Priority::Medium,
            };
            commands::add_task(description.clone(), p);
        }
        Commands::List => {
            commands::list_tasks();
        }
        Commands::Done { id } => {
            commands::mark_task_done(*id);
        }
    }
}

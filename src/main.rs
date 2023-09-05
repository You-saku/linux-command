use clap::{Parser, Subcommand};
use walkdir::{DirEntry, WalkDir};

/// rust cli tool like linux command
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Ls {},
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn read_dir() {
    WalkDir::new(".")
        .max_depth(1)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| {
            let path_str: String = x.path().to_str().unwrap().replace("./", "");
            match &*path_str {
                "." => print!(""),
                _ => println!("{path_str}"),
            }
        });
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Ls {}) => read_dir(),
        None => {
            println!("you sohuld enter command.")
        }
    }
}

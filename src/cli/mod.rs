use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub path: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(alias = "a", about = "Add a new note")]
    Add,

    #[command(alias = "e", about = "Edit a note")]
    Edit,

    #[command(alias = "f", about = "Find notes")]
    Find {
        #[arg(long, help = "Regular expression to search for")]
        re: Option<String>,

        #[arg(long, help = "Keywords to search for")]
        kw: Option<String>,
    },

    #[command(alias = "l", alias = "ls", about = "List notes")]
    List,

    #[command(alias = "r", alias = "rm", about = "Remove note(s)")]
    Remove,
}

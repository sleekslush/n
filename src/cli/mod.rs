pub mod commands;

use clap::{ArgGroup, Parser, Subcommand};
use uuid::Uuid;

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
    Add(AddArgs),

    #[command(alias = "e", about = "Edit a note")]
    Edit,

    #[command(alias = "f", about = "Find notes", group(
        ArgGroup::new("find_by")
            .args(&["ids", "uuids", "re", "kw"])
            .multiple(false)
    ))]
    Find(FindArgs),

    #[command(alias = "r", alias = "rm", about = "Remove note(s)")]
    Remove,
}

#[derive(Parser)]
pub struct AddArgs {
    #[arg(trailing_var_arg = true, num_args = 1.., help = "The note text")]
    note: String,
}

#[derive(Parser)]
pub struct FindArgs {
    #[arg(long, value_delimiter = ',', help = "Find by id")]
    ids: Option<Vec<i32>>,

    #[arg(long, value_delimiter = ',', help = "Find by uuid")]
    uuids: Option<Vec<Uuid>>,

    #[arg(long, help = "Regular expression to search for")]
    re: Option<String>,

    #[arg(long, help = "Keywords to search for")]
    kw: Option<String>,
}

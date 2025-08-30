pub mod commands;

use clap::{ArgGroup, Parser, Subcommand, ValueEnum};
use uuid::Uuid;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub path: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(alias = "a", about = "Add a new note")]
    Add(AddArgs),

    #[command(alias = "e", about = "Edit a note")]
    Edit(EditArgs),

    #[command(alias = "f", about = "Find notes", group(
        ArgGroup::new("find_by")
            .args(&["uuid", "re", "kw"])
            .multiple(false)
    ))]
    Find(FindArgs),

    #[command(alias = "r", alias = "rm", about = "Remove note(s)")]
    Remove(RemoveArgs),
}

#[derive(Parser)]
pub struct AddArgs {
    #[arg(trailing_var_arg = true, num_args = 1.., help = "The note text")]
    note: Option<Vec<String>>,
}

#[derive(Parser)]
pub struct EditArgs {
    #[arg(help = "The UUID of the note to edit")]
    uuid: Option<Uuid>,
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputField {
    Uuid,
    Note,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Json,
    Text,
}

#[derive(Parser)]
pub struct FindArgs {
    #[arg(long, value_delimiter = ',', help = "Find by id or uuid")]
    uuid: Option<Vec<Uuid>>,

    #[arg(long, help = "Regular expression to search for")]
    re: Option<String>,

    #[arg(long, help = "Keywords to search for")]
    kw: Option<String>,

    #[arg(
        long,
        value_delimiter = ',',
        default_value = "uuid,note",
        help = "Fields to return"
    )]
    fields: Option<Vec<OutputField>>,

    #[arg(long, help = "Output format", default_value = "text")]
    format: OutputFormat,
}

#[derive(Parser)]
pub struct RemoveArgs {
    #[arg(required = true, num_args = 1.., trailing_var_arg = true, help = "UUIDs to remove")]
    uuid: Vec<Uuid>,
}

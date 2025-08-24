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

    #[command(alias = "e", about = "Edit a note"/* group(
        ArgGroup::new("edit_by")
            .args(&["id", "uuid"])
            .multiple(false)
            .required(true)
    )*/)]
    Edit(EditArgs),

    #[command(alias = "f", about = "Find notes", group(
        ArgGroup::new("find_by")
            .args(&["ids", "uuids", "re", "kw"])
            .multiple(false)
    ))]
    Find(FindArgs),

    #[command(alias = "r", alias = "rm", about = "Remove note(s)")]
    Remove,
    //#[command(about = "View a note")]
    //View(ViewArgs),
}

#[derive(Parser)]
pub struct AddArgs {
    #[arg(trailing_var_arg = true, num_args = 1.., help = "The note text")]
    note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum IdOrUuid {
    Id(i32),
    Uuid(Uuid),
}

impl std::str::FromStr for IdOrUuid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(i) = s.parse::<i32>() {
            Ok(IdOrUuid::Id(i))
        } else if let Ok(u) = s.parse::<Uuid>() {
            Ok(IdOrUuid::Uuid(u))
        } else {
            Err(format!("`{s}` is neither an integer nor a UUID"))
        }
    }
}

#[derive(Parser)]
pub struct EditArgs {
    #[arg(help = "The id or UUID of the note to edit")]
    id: IdOrUuid,
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

#[derive(Parser)]
pub struct ViewArgs {}

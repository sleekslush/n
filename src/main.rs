mod cli;
mod database;

use std::{fs, path::Path};

use clap::Parser;
use cli::{Cli, Commands};
use rusqlite::Connection;
use uuid::Uuid;

use crate::database::{connection::DatabaseConnection, models::Note, repository::NoteRepository};

fn main() {
    let cli = Cli::parse();

    // Default path to notes database or override with --path
    let notes_path_buf = if let Some(ref path_str) = cli.path {
        Path::new(path_str).to_path_buf()
    } else {
        let expanded_path = shellexpand::tilde("~/.n/notes.db").to_string();
        let default_path = Path::new(&expanded_path).to_path_buf();
        fs::create_dir_all(default_path.parent().unwrap()).ok();
        default_path
    };

    let mut conn = Connection::open(&notes_path_buf).expect("Failed to open the database");

    let mut db_conn =
        DatabaseConnection::new(&mut conn).expect("Failed to connect to the database");
    db_conn.migrate();

    let note_repo = NoteRepository::new(&mut db_conn);

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Add) => {
            let note = Note {
                _id: None,
                uuid: Uuid::new_v4().to_string(),
                created_at: chrono::Utc::now().naive_utc().to_string(),
                updated_at: chrono::Utc::now().naive_utc().to_string(),
                note: String::from("This is a new note"),
            };

            note_repo.create_note(&note);

            println!("Adding note {:?}", note.uuid);
        }
        Some(Commands::Edit) => {
            println!("Editing notes...");
        }
        Some(Commands::Find { re: _, kw: _ }) => {
            println!("Finding notes...");
        }
        Some(Commands::List) => {
            println!("Listing notes...");
        }
        Some(Commands::Remove) => {
            println!("Removing notes...");
        }
        None => {
            println!("Open an editor to add notes...");
        }
    }
}

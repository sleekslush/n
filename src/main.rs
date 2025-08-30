mod cli;
mod database;
mod format;

use clap::Parser;
use cli::{Cli, Commands};

use crate::{
    cli::commands::{add::add_note, edit::edit_note, find::find_notes, remove::remove_notes},
    database::{connection::get_db_connection, migration::migrate, repository::NoteRepository},
};

fn main() {
    // Parse command line arguments
    let cli = Cli::parse();

    // Open or create the database and run migrations
    let mut db_conn = get_db_connection(cli.path).expect("Failed to open the database");
    migrate(&mut db_conn);

    // Note repository to handle note operations
    let note_repo = NoteRepository::new(&mut db_conn);

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add(args) => add_note(&note_repo, args),
        Commands::Edit(args) => edit_note(&note_repo, args),
        Commands::Find(args) => find_notes(&note_repo, args),
        Commands::Remove(args) => remove_notes(&note_repo, args),
    }
}

use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
};
use tempfile::NamedTempFile;

use crate::{
    cli::{EditArgs, IdOrUuid},
    database::{models::Note, repository::NoteRepository},
};

pub fn edit_note(repo: &NoteRepository, args: &EditArgs) {
    let note = match args.id {
        IdOrUuid::Id(id) => repo.get_note_by_id(id),
        IdOrUuid::Uuid(uuid) => repo.get_note_by_uuid(uuid),
    };

    match note {
        Ok(note) => edit_in_editor(note, repo),
        Err(e) => println!("Failed to find note: {}", e),
    }
}

fn edit_in_editor(mut note: Note, note_repo: &NoteRepository) {
    let mut tmp_file = NamedTempFile::new().expect("Failed to create temp file");
    tmp_file
        .write_all(note.note.as_bytes())
        .expect("Failed to write to temp file");

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    let status = std::process::Command::new(editor)
        .arg(tmp_file.path())
        .status()
        .expect("Failed to open editor");

    if status.success() {
        // Re-open the file from disk
        let mut file = File::open(tmp_file.path()).expect("Failed to open temp file after edit");

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read temp file");

        // Only update if the content has changed
        if contents != note.note {
            note.note = contents;
            note_repo.update_note(&note).expect("Failed to update note");
        }
    }
}

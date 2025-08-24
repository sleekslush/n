use std::{
    fs::File,
    io::{Read, Write},
};
use tempfile::NamedTempFile;

use crate::{
    cli::{EditArgs, IdOrUuid},
    database::repository::NoteRepository,
};

pub fn edit_note(repo: &NoteRepository, args: &EditArgs) {
    let note = match args.id {
        Some(IdOrUuid::Id(id)) => repo.get_note_by_id(id),
        Some(IdOrUuid::Uuid(uuid)) => repo.get_note_by_uuid(uuid),
        None => repo.get_last_updated_note(),
    };

    match note {
        Ok(mut note) => {
            let original_note = note.note;

            if let Some(edited_note) = edit_in_editor(Some(original_note.clone()))
                && !edited_note.trim().is_empty()
                && edited_note != original_note
            {
                note.note = edited_note;
                repo.update_note(&note).expect("Failed to update note");
            }
        }
        Err(e) => println!("Failed to find note: {}", e),
    }
}

pub fn edit_in_editor(note: Option<String>) -> Option<String> {
    let mut tmp_file = NamedTempFile::new().expect("Failed to create temp file");

    if let Some(ref note) = note {
        tmp_file
            .write_all(note.as_bytes())
            .expect("Failed to write to temp file");
    }

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    let status = std::process::Command::new(editor)
        .arg(tmp_file.path())
        .status()
        .expect("Failed to open editor");

    if !status.success() {
        note
    } else {
        // Re-open the file from disk
        let mut file = File::open(tmp_file.path()).expect("Failed to open temp file after edit");

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read temp file");

        Some(contents)
    }
}

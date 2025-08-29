use crate::{
    cli::{AddArgs, commands::edit::edit_in_editor},
    database::repository::NoteRepository,
};

pub fn add_note(repo: &NoteRepository, args: &AddArgs) {
    if let Some(note_text) = &args.note {
        let note = note_text.join(" ");
        if note.trim().is_empty() {
            panic!("Note cannot be empty");
        }

        match repo.create_note(note.clone()) {
            Ok(note) => println!("{}", note.uuid),
            Err(e) => panic!("Failed to add note: {}", e),
        };
    } else {
        add_editor_note(repo)
    }
}

pub fn add_editor_note(repo: &NoteRepository) {
    if let Some(note) = edit_in_editor(None)
        && !note.trim().is_empty()
    {
        match repo.create_note(note) {
            Ok(note) => println!("{}", note.uuid),
            Err(e) => panic!("Failed to add note: {}", e),
        };
    }
}

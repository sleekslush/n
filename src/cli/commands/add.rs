use crate::{cli::AddArgs, database::repository::NoteRepository};

pub fn add_note(repo: &NoteRepository, args: &AddArgs) {
    if args.note.trim().is_empty() {
        panic!("Note cannot be empty");
    }

    match repo.create_note(args.note.clone()) {
        Ok(note) => println!(
            "Adding note\n\tid: {}\n\tuuid: {}\n\tcreated_at: {}\n\tupdated_at: {}\n\tnote: {}",
            note.id, note.uuid, note.created_at, note.updated_at, note.note
        ),
        Err(e) => panic!("Failed to add note: {}", e),
    };
}

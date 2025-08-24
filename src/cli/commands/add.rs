use crate::{cli::AddArgs, database::repository::NoteRepository};

pub fn add_note(repo: &NoteRepository, args: &AddArgs) {
    if args.note.trim().is_empty() {
        panic!("Note cannot be empty");
    }

    match repo.create_note(args.note.clone()) {
        Ok(note) => println!("{}", note.uuid),
        Err(e) => panic!("Failed to add note: {}", e),
    };
}

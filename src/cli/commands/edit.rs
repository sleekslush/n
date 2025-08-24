use crate::{
    cli::{EditArgs, IdOrUuid},
    database::repository::NoteRepository,
};

pub fn edit_note(repo: &NoteRepository, args: &EditArgs) {
    let note = match args.id {
        IdOrUuid::Id(id) => repo.get_note_by_id(id),
        IdOrUuid::Uuid(uuid) => repo.get_note_by_uuid(uuid),
    };

    match note {
        Ok(note) => println!(
            "Editing note\n\tid: {}\n\tuuid: {}\n\tcreated_at: {}\n\tupdated_at: {}\n\tnote: {}",
            note.id, note.uuid, note.created_at, note.updated_at, note.note
        ),
        Err(e) => println!("Failed to find note: {}", e),
    }
}

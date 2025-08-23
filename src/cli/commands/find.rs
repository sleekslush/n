use crate::{cli::FindArgs, database::repository::NoteRepository};

pub fn find_notes(repo: &NoteRepository, args: &FindArgs) {
    if let Some(ids) = &args.ids {
        match repo.get_notes_by_ids(ids) {
            Ok(notes) => notes.iter().for_each(|note| {
                println!(
                    "Found note\n\tid: {}\n\tuuid: {}\n\tcreated_at: {}\n\tupdated_at: {}\n\tnote: {}",
                    note.id, note.uuid, note.created_at, note.updated_at, note.note
                )
            }),
            Err(e) => println!("Failed to find note by id {}: {}", ids[0], e),
        }
    } else if let Some(uuids) = &args.uuids {
        match repo.get_note_by_uuid(uuids[0]) {
            Ok(note) => println!(
                "Found note\n\tid: {}\n\tuuid: {}\n\tcreated_at: {}\n\tupdated_at: {}\n\tnote: {}",
                note.id, note.uuid, note.created_at, note.updated_at, note.note
            ),
            Err(e) => println!("Failed to find note by uuid {}: {}", uuids[0], e),
        }
    } else if let Some(re) = &args.re {
        println!("Searching notes with regex: {}", re);
    } else if let Some(kw) = &args.kw {
        println!("Searching notes with keywords: {}", kw);
    } else {
        println!("No search criteria provided.");
    }
}

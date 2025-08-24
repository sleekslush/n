use crate::{cli::FindArgs, database::repository::NoteRepository};

pub fn find_notes(repo: &NoteRepository, args: &FindArgs) {
    let notes = if let Some(ids) = &args.ids {
        repo.get_notes_by_ids(ids)
    } else if let Some(uuids) = &args.uuids {
        repo.get_notes_by_uuids(uuids)
    } else if let Some(_) = &args.re {
        // Placeholder for regex search implementation
        unimplemented!("Regex search not implemented yet");
    } else if let Some(_) = &args.kw {
        // Placeholder for keyword search implementation
        unimplemented!("Keyword search not implemented yet");
    } else {
        repo.get_latest_notes(100)
    };

    match notes {
        Ok(notes) => {
            for note in notes {
                println!("{:?}", note);
            }
        }
        Err(e) => println!("Failed to find notes: {}", e),
    }
}

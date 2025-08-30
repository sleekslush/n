use std::collections::HashSet;

use uuid::Uuid;

use crate::{cli::RemoveArgs, database::repository::NoteRepository};

pub fn remove_notes(repo: &NoteRepository, args: &RemoveArgs) {
    let removed_uuids = repo.delete_notes(&args.uuid);
    let removed_set: HashSet<Uuid> = removed_uuids.into_iter().flatten().collect();
    let input_set: HashSet<Uuid> = args.uuid.iter().cloned().collect();
    for uuid in input_set.difference(&removed_set) {
        eprintln!("{}: no such uuid", uuid);
    }
}

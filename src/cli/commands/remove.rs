use std::collections::HashSet;

use uuid::Uuid;

use crate::{cli::RemoveArgs, database::repository::NoteRepository};

pub fn remove_notes(repo: &NoteRepository, args: &RemoveArgs) {
    let removed_uuids = repo.delete_notes(&args.uuid);

    let input_set: HashSet<Uuid> = args.uuid.iter().cloned().collect();
    let removed_set = removed_uuids.into_iter().flatten().collect();
    let unremoved_set = input_set.difference(&removed_set);

    for uuid in unremoved_set {
        eprintln!("{}: no such uuid", uuid);
    }
}

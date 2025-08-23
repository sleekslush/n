use crate::database::connection::DatabaseConnection;

pub fn migrate<'a>(connection: &'a mut DatabaseConnection) {
    let user_version = connection
        .connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .unwrap_or(0);

    if user_version >= MIGRATIONS.len() {
        return;
    }

    let tx = connection.connection.transaction().unwrap();

    for group in MIGRATIONS.iter().rev().skip(user_version) {
        for &migration in group.iter() {
            tx.execute(migration, ()).unwrap();
        }
    }

    tx.execute(&format!("PRAGMA user_version = {}", MIGRATIONS.len()), [])
        .unwrap();

    tx.commit().unwrap();
}

// Groups of migrations execute from bottom to top, and then top down within each group
const MIGRATIONS: &[&[&str]] = &[&[
    r"CREATE TABLE notes (
        id INTEGER PRIMARY KEY,
        uuid TEXT NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL,
        note TEXT NOT NULL
    )",
    r"CREATE VIRTUAL TABLE notes_ft USING fts5(
        note,
        content='notes',
        content_rowid='id'
    )",
    r"CREATE TRIGGER notes_ai AFTER INSERT ON notes BEGIN
        INSERT INTO notes_ft(rowid, note) VALUES (new.id, new.note);
    END",
    r"CREATE TRIGGER notes_ad AFTER DELETE ON notes BEGIN
        INSERT INTO notes_ft(notes_ft, rowid, note) VALUES('delete', old.id, old.note);
    END",
    r"CREATE TRIGGER notes_au AFTER UPDATE ON notes BEGIN
        INSERT INTO notes_ft(notes_ft, rowid, note) VALUES('delete', old.id, old.note);
        INSERT INTO notes_ft(rowid, note) VALUES (new.id, new.note);
    END",
]];

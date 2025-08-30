CREATE TABLE notes (
    uuid TEXT NOT NULL PRIMARY KEY,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    note TEXT NOT NULL
);

CREATE VIRTUAL TABLE notes_ft USING fts5(
    note,
    content='notes',
    content_rowid='uuid'
);

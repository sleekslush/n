CREATE TABLE notes (
    id INTEGER PRIMARY KEY,
    uuid TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    note TEXT NOT NULL
);

CREATE VIRTUAL TABLE notes_ft USING fts5(
    note,
    content='notes',
    content_rowid='id'
);

CREATE TRIGGER notes_ai AFTER INSERT ON notes BEGIN
    INSERT INTO notes_ft(rowid, note) VALUES (new.id, new.note);
END;

CREATE TRIGGER notes_ad AFTER DELETE ON notes BEGIN
    INSERT INTO notes_ft(notes_ft, rowid, note) VALUES('delete', old.id, old.note);
END;

CREATE TRIGGER notes_au AFTER UPDATE ON notes BEGIN
    INSERT INTO notes_ft(notes_ft, rowid, note) VALUES('delete', old.id, old.note);
    INSERT INTO notes_ft(rowid, note) VALUES (new.id, new.note);
END;
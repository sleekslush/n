use chrono::Utc;
use rusqlite::{Connection, Result, params, params_from_iter};
use uuid::Uuid;

use crate::database::{
    models::Note,
    types::{SqliteUTC, SqliteUuid},
};

pub struct NoteRepository<'a> {
    connection: &'a Connection,
}

impl<'a> NoteRepository<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }

    pub fn create_note(&self, note: String) -> Result<Note> {
        let uuid = SqliteUuid(Uuid::new_v4());
        let now = SqliteUTC(Utc::now());

        self.connection.query_row(
            "
                    INSERT INTO notes (uuid, created_at, updated_at, note)
                    VALUES (?1, ?2, ?3, ?4)
                    RETURNING id, uuid, created_at, updated_at, note
                ",
            params![uuid, now, now, note],
            |row| note_from_row(row),
        )
    }

    pub fn update_note(&self, note: &Note) -> Result<Note> {
        let now = SqliteUTC(Utc::now());

        self.connection.query_row(
            "
                UPDATE notes
                SET updated_at = ?1, note = ?2
                WHERE id = ?3
                RETURNING id, uuid, created_at, updated_at, note
            ",
            params![now, note.note, note.id],
            |row| note_from_row(row),
        )
    }

    pub fn get_note_by_id(&self, note_id: i32) -> Result<Note> {
        self.connection.query_row(
            "
                SELECT id, uuid, created_at, updated_at, note
                FROM notes
                WHERE id = ?1
            ",
            [note_id],
            |row| note_from_row(row),
        )
    }

    pub fn get_notes_by_ids(&self, note_ids: &[i32]) -> Result<Vec<Note>> {
        if note_ids.is_empty() {
            return Ok(vec![]);
        }

        self.connection
            .prepare(&format!(
                "
                    SELECT id, uuid, created_at, updated_at, note
                    FROM notes
                    WHERE id IN ({})
                ",
                n_placeholders(note_ids.len())
            ))?
            .query_map(params_from_iter(note_ids), |row| note_from_row(row))?
            .collect()
    }

    pub fn get_note_by_uuid(&self, note_uuid: Uuid) -> Result<Note> {
        self.connection.query_row(
            "
                SELECT id, uuid, created_at, updated_at, note
                FROM notes
                WHERE uuid = ?1
            ",
            [SqliteUuid(note_uuid)],
            |row| note_from_row(row),
        )
    }

    /*pub fn get_notes_by_uuids(&self, note_uuids: &[Uuid]) -> Result<Vec<Note>> {
        if note_uuids.is_empty() {
            return Ok(vec![]);
        }

        let sql_uuids: Vec<SqliteUuid> = note_uuids.iter().cloned().map(SqliteUuid).collect();

        self.connection
            .prepare(&format!(
                "
                    SELECT id, uuid, created_at, updated_at, note
                    FROM notes
                    WHERE uuid IN ({})
                ",
                n_placeholders(sql_uuids.len())
            ))?
            .query_map(params_from_iter(sql_uuids), |row| note_from_row(row))?
            .collect()
    }

    pub fn get_latest_note(&self) -> Result<Note> {
        self.connection.query_one(
            "
                SELECT id, uuid, created_at, updated_at, note
                FROM notes
                ORDER BY created_at DESC
                LIMIT 1
            ",
            [],
            |row| note_from_row(row),
        )
    }

    pub fn get_last_updated_note(&self) -> Result<Note> {
        self.connection.query_one(
            "
                SELECT id, uuid, created_at, updated_at, note
                FROM notes
                ORDER BY updated_at DESC
                LIMIT 1
            ",
            [],
            |row| note_from_row(row),
        )
    }*/
}

fn note_from_row(row: &rusqlite::Row) -> Result<Note> {
    Ok(Note {
        id: row.get("id")?,
        uuid: row
            .get::<_, SqliteUuid>("uuid")
            .map(|SqliteUuid(uuid)| uuid)?,
        created_at: row
            .get::<_, SqliteUTC>("created_at")
            .map(|SqliteUTC(dt)| dt)?,
        updated_at: row
            .get::<_, SqliteUTC>("updated_at")
            .map(|SqliteUTC(dt)| dt)?,
        note: row.get(4)?,
    })
}

fn n_placeholders(n: usize) -> String {
    (0..n).map(|_| "?").collect::<Vec<_>>().join(",")
}

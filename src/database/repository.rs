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
                VALUES (?1, ?2, ?2, ?3)
                RETURNING *
            ",
            params![uuid, now, note],
            |row| note_from_row(row),
        )
    }

    pub fn update_note(&self, note: &Note) -> Result<Note> {
        let uuid = SqliteUuid(note.uuid);
        let now = SqliteUTC(Utc::now());

        self.connection.query_row(
            "
                UPDATE notes
                SET updated_at = ?1, note = ?2
                WHERE uuid = ?3
                RETURNING *
            ",
            params![now, note.note, uuid],
            |row| note_from_row(row),
        )
    }

    pub fn get_note_by_uuid(&self, uuid: Uuid) -> Result<Note> {
        self.connection.query_row(
            "
                SELECT *
                FROM notes
                WHERE uuid = ?1
            ",
            [SqliteUuid(uuid)],
            |row| note_from_row(row),
        )
    }

    pub fn get_notes_by_uuids(&self, uuids: &[Uuid]) -> Result<Vec<Note>> {
        if uuids.is_empty() {
            return Ok(vec![]);
        }

        let sql_uuids: Vec<SqliteUuid> = uuids.iter().cloned().map(SqliteUuid).collect();

        self.connection
            .prepare(&format!(
                "
                    SELECT *
                    FROM notes
                    WHERE uuid IN ({})
                ",
                n_placeholders(sql_uuids.len())
            ))?
            .query_map(params_from_iter(sql_uuids), |row| note_from_row(row))?
            .collect()
    }

    pub fn get_latest_notes(&self, limit: usize) -> Result<Vec<Note>> {
        self.connection
            .prepare(
                "
                SELECT *
                FROM notes
                ORDER BY updated_at DESC
                LIMIT ?1
            ",
            )?
            .query_map([limit as i64], |row| note_from_row(row))?
            .collect()
    }

    pub fn get_last_updated_note(&self) -> Result<Note> {
        self.connection.query_one(
            "
                SELECT *
                FROM notes
                ORDER BY updated_at DESC
                LIMIT 1
            ",
            [],
            |row| note_from_row(row),
        )
    }

    pub fn delete_notes(&self, uuids: &[Uuid]) -> Result<Vec<Uuid>> {
        let sql_uuids: Vec<SqliteUuid> = uuids.iter().cloned().map(SqliteUuid).collect();

        self.connection
            .prepare(&format!(
                "
                DELETE FROM notes
                WHERE uuid IN ({})
                RETURNING uuid
            ",
                n_placeholders(uuids.len())
            ))?
            .query_map(params_from_iter(sql_uuids), |row| {
                row.get::<_, SqliteUuid>("uuid")
                    .map(|SqliteUuid(uuid)| uuid)
            })?
            .collect()
    }
}

fn note_from_row(row: &rusqlite::Row) -> Result<Note> {
    Ok(Note {
        uuid: row
            .get::<_, SqliteUuid>("uuid")
            .map(|SqliteUuid(uuid)| uuid)?,
        created_at: row
            .get::<_, SqliteUTC>("created_at")
            .map(|SqliteUTC(dt)| dt)?,
        updated_at: row
            .get::<_, SqliteUTC>("updated_at")
            .map(|SqliteUTC(dt)| dt)?,
        note: row.get("note")?,
    })
}

fn n_placeholders(n: usize) -> String {
    (0..n).map(|_| "?").collect::<Vec<_>>().join(",")
}

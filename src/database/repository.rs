use crate::database::{connection::DatabaseConnection, models::Note};

pub struct NoteRepository<'a> {
    connection: &'a DatabaseConnection<'a>,
}

impl<'a> NoteRepository<'a> {
    pub fn new(connection: &'a DatabaseConnection<'a>) -> Self {
        Self { connection }
    }

    pub fn create_note(&self, note: &Note) {
        self.connection
            .connection
            .execute(
                r"INSERT INTO notes (uuid, created_at, updated_at, note) VALUES (?1, ?2, ?3, ?4)",
                &[&note.uuid, &note.created_at, &note.updated_at, &note.note],
            )
            .expect("Failed to insert note");
    }

    /*fn get_note_by_id(&self, note_id: i32) -> Result<Note, DatabaseError> {
        // Implementation for retrieving a note by its ID
        Ok(Note::default())
    }

    fn update_note(&self, note: &Note) -> Result<(), DatabaseError> {
        // Implementation for updating a note in the database
        Ok(())
    }

    fn delete_note(&self, note_id: i32) -> Result<(), DatabaseError> {
        // Implementation for deleting a note from the database
        Ok(())
    }*/
}

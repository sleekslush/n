use rusqlite::Connection;

use crate::database::migration::migrate;

pub struct DatabaseConnection<'a> {
    pub connection: &'a mut Connection,
}

impl<'a> DatabaseConnection<'a> {
    pub fn new(connection: &'a mut Connection) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { connection })
    }

    pub fn migrate(&mut self) {
        migrate(self);
    }
}

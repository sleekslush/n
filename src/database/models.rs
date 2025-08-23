use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Note {
    pub id: i32,
    pub uuid: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub note: String,
}

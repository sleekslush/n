#[derive(Debug)]
pub struct Note {
    pub _id: Option<i32>,
    pub uuid: String,
    pub created_at: String,
    pub updated_at: String,
    pub note: String,
}

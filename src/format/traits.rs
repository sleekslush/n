use crate::database::models::Note;

pub trait OutputFormatter {
    fn format(&self, note: &Note) -> String;
}

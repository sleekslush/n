use crate::{database::models::Note, format::traits::OutputFormatter};

pub struct TextFormatter;

impl OutputFormatter for TextFormatter {
    fn format(&self, note: &Note) -> String {
        format!(
            "{uuid}\t{note}",
            uuid = note.uuid,
            note = note.note.replace("\r\n", "\\r\\n").replace("\n", "\\n")
        )
    }
}

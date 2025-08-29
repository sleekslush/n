use crate::{database::models::Note, format::traits::OutputFormatter};

pub struct JsonFormatter;

impl OutputFormatter for JsonFormatter {
    fn format(&self, note: &Note) -> String {
        serde_json::to_string(note).unwrap()
    }
}

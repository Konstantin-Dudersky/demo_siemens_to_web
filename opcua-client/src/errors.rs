use chrono::ParseError;

#[derive(Debug)]
pub enum Errors {
    ConvertDateTimeToChrono(String),
}

impl From<ParseError> for Errors {
    fn from(value: ParseError) -> Self {
        Self::ConvertDateTimeToChrono(value.to_string())
    }
}

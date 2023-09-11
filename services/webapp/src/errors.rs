#[derive(Debug)]
pub enum Errors {
    /// Не определили адрес окна
    WindowLocationError(String),
}

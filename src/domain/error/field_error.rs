#[derive(Debug, Clone)]
pub struct FieldError {
    pub field: &'static str,
    pub message: String,
}

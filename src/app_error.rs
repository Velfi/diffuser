use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("No element in list \"{list_name}\" at index ({x}, {y})")]
    InvalidXyIndex {
        list_name: String,
        x: usize,
        y: usize,
    },
}

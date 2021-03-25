use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("No element in list \"{list_name}\" at index ({x}, {y}), list is {len} elements long")]
    InvalidXyIndex {
        list_name: String,
        x: usize,
        y: usize,
        len: usize,
    },

    #[error("No element in list \"{list_name}\" at index {index}, list is {len} elements long")]
    InvalidIndex {
        list_name: String,
        index: usize,
        len: usize,
    },
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("No element in list \"{list_name}\" at index {index}, list is {len} elements long")]
    InvalidIndex {
        list_name: String,
        index: usize,
        len: usize,
    },
}

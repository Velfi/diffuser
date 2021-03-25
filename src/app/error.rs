use thiserror::Error as ErrorDerive;

#[derive(Debug, ErrorDerive)]
pub enum Error {
    #[error("No element in list \"{list_name}\" at index {index}, list is {len} elements long")]
    InvalidIndex {
        list_name: String,
        index: usize,
        len: usize,
    },
}

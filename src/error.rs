#[derive(thiserror::Error, Debug)]
pub enum InputError {
    #[error("{0}: command not found")]
    CommandNotFound(String),
}

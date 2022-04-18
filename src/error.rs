use thiserror::Error;

#[derive(Error, Debug)]
pub enum OmseError {
    #[error("No VLC source video URL available yet, please try again")]
    NoUrlAvailable,
    #[error("No scene by the name {0} found")]
    NoSourceFound(String)
}
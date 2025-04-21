use thiserror::Error;

#[derive(Error, Debug)]
pub enum PigmentsError {
    #[error("Failed to process image: {0}")]
    ImageProcessError(String),

    #[error("Failed to extract colors: {0}")]
    ColorExtractionError(String),

    #[error("Invalid number of colors requested: {0}")]
    InvalidColorCount(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, PigmentsError>; 
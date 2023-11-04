use std::num::ParseIntError;

use color_eyre::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AoCError {
    #[error("Failed to install Color Eyre")]
    ColorEyreError(#[from] Report),

    #[error("Failed reading from file")]
    FileReadError(#[from] std::io::Error),

    #[error("Failed to parse integer")]
    ParseIntError(#[from] ParseIntError),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum RustBucketError {
    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid color format: {0}")]
    InvalidColor(String),

    #[error("Palette not found: {0}")]
    PaletteNotFound(String),

    #[error("Invalid pixel area: {0}")]
    InvalidPixelArea(String),

    #[error("Color not found in palette: {0}")]
    ColorNotFound(String),

    #[error("Palette parse error: {0}")]
    PaletteParseError(String),
}

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, RustBucketError>;

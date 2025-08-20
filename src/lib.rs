pub mod cli;
pub mod error;
pub mod image;
pub mod palette;

pub use cli::Args;
pub use error::{Result, RustBucketError};
pub use image::{ImageBenchmark, ImagePipeline, ImageProcessor, OptimizationSuggestions};
pub use palette::{PaletteLoader, PaletteManager};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Color {
    pub name: String,
    pub hex: String,
}

#[derive(Debug, Clone)]
pub struct Palette {
    pub name: String,
    pub path: PathBuf,
    pub colors: Vec<Color>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub input_path: Option<PathBuf>,
    pub output_path: PathBuf,
    pub palette: String,
    pub colors: Vec<String>,
    pub enable_blur: bool,
    pub disable_avg_pixels: bool,
    pub pixels_area: Option<(u32, u32)>,
    pub quiet_mode: bool,
    pub benchmark: bool,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Config {
            input_path: args.input_path,
            output_path: args.output_path,
            palette: args.palette,
            colors: args.colors,
            enable_blur: args.enable_blur,
            disable_avg_pixels: args.disable_avg_pixels,
            pixels_area: args.pixels_area,
            quiet_mode: args.quiet_mode,
            benchmark: args.benchmark,
        }
    }
}

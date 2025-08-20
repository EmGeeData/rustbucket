mod benchmark;
mod converter;
mod effects;
mod processor;

pub use benchmark::{BenchmarkResult, ImageBenchmark, OptimizationSuggestions};
pub use converter::{NearestColorConverter, PaletteConverter};
pub use effects::{BlurConfig, NoiseReduction};
pub use processor::{ImagePipeline, ImageProcessor};

use crate::RustBucketError;
use image::{DynamicImage, Rgb};
use std::path::Path;

pub fn load_image<P: AsRef<Path>>(path: P) -> crate::Result<DynamicImage> {
    let img = image::open(path.as_ref()).map_err(RustBucketError::ImageError)?;

    log::info!("Loaded image: {}x{} pixels", img.width(), img.height());
    Ok(img)
}

pub fn save_image<P: AsRef<Path>>(img: &DynamicImage, path: P) -> crate::Result<()> {
    img.save(path.as_ref())
        .map_err(RustBucketError::ImageError)?;

    log::info!("Saved image to: {}", path.as_ref().display());
    Ok(())
}

pub fn rgb_distance(color1: &Rgb<u8>, color2: &Rgb<u8>) -> f64 {
    let r1 = f64::from(color1[0]);
    let g1 = f64::from(color1[1]);
    let b1 = f64::from(color1[2]);

    let r2 = f64::from(color2[0]);
    let g2 = f64::from(color2[1]);
    let b2 = f64::from(color2[2]);

    // Euclidean distance in RGB space
    ((r1 - r2).powi(2) + (g1 - g2).powi(2) + (b1 - b2).powi(2)).sqrt()
}

pub fn hex_to_rgb(hex: &str) -> crate::Result<Rgb<u8>> {
    let (r, g, b) = crate::palette::hex_to_rgb(hex)?;
    Ok(Rgb([r, g, b]))
}

pub fn rgb_to_hex(rgb: &Rgb<u8>) -> String {
    crate::palette::rgb_to_hex(rgb[0], rgb[1], rgb[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_distance() {
        let black = Rgb([0, 0, 0]);
        let white = Rgb([255, 255, 255]);
        let red = Rgb([255, 0, 0]);

        assert_eq!(rgb_distance(&black, &black), 0.0);
        assert!(rgb_distance(&black, &white) > rgb_distance(&black, &red));
    }

    #[test]
    fn test_hex_to_rgb_conversion() {
        let rgb = hex_to_rgb("#BF616A").unwrap();
        assert_eq!(rgb, Rgb([191, 97, 106]));

        let rgb = hex_to_rgb("FFFFFF").unwrap();
        assert_eq!(rgb, Rgb([255, 255, 255]));
    }

    #[test]
    fn test_rgb_to_hex_conversion() {
        let hex = rgb_to_hex(&Rgb([191, 97, 106]));
        assert_eq!(hex, "#BF616A");

        let hex = rgb_to_hex(&Rgb([0, 0, 0]));
        assert_eq!(hex, "#000000");
    }
}

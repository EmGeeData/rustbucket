use crate::Palette;
use image::{DynamicImage, ImageBuffer, Rgb, RgbImage};
use std::collections::HashMap;

use super::{hex_to_rgb, rgb_distance};

pub trait PaletteConverter {
    fn convert_pixel(&self, rgb: Rgb<u8>) -> Rgb<u8>;
    fn convert_image(&self, img: &DynamicImage) -> DynamicImage;
}

pub struct NearestColorConverter {
    palette_colors: Vec<Rgb<u8>>,
    color_cache: HashMap<Rgb<u8>, Rgb<u8>>,
}

impl NearestColorConverter {
    pub fn new(palette: &Palette) -> crate::Result<Self> {
        let mut palette_colors = Vec::new();

        for color in &palette.colors {
            let rgb = hex_to_rgb(&color.hex)?;
            palette_colors.push(rgb);
        }

        if palette_colors.is_empty() {
            return Err(crate::RustBucketError::PaletteNotFound(format!(
                "Palette '{}' has no valid colors",
                palette.name
            )));
        }

        log::info!(
            "Created converter with {} palette colors",
            palette_colors.len()
        );

        Ok(Self {
            palette_colors,
            color_cache: HashMap::new(),
        })
    }

    pub fn new_with_selected_colors(
        palette: &Palette,
        selected_colors: &[String],
    ) -> crate::Result<Self> {
        let mut palette_colors = Vec::new();

        if selected_colors.is_empty() {
            return Self::new(palette);
        }

        for color_name in selected_colors {
            if let Some(color) = palette.colors.iter().find(|c| c.name == *color_name) {
                let rgb = hex_to_rgb(&color.hex)?;
                palette_colors.push(rgb);
            } else {
                log::warn!(
                    "Selected color '{}' not found in palette '{}'",
                    color_name,
                    palette.name
                );
            }
        }

        if palette_colors.is_empty() {
            return Err(crate::RustBucketError::ColorNotFound(format!(
                "None of the selected colors found in palette '{}'",
                palette.name
            )));
        }

        log::info!(
            "Created converter with {} selected colors from palette",
            palette_colors.len()
        );

        Ok(Self {
            palette_colors,
            color_cache: HashMap::new(),
        })
    }

    fn find_nearest_color(&self, target: Rgb<u8>) -> Rgb<u8> {
        let mut min_distance = f64::INFINITY;
        let mut nearest_color = self.palette_colors[0];

        for &palette_color in &self.palette_colors {
            let distance = rgb_distance(&target, &palette_color);
            if distance < min_distance {
                min_distance = distance;
                nearest_color = palette_color;
            }
        }

        nearest_color
    }

    pub fn clear_cache(&mut self) {
        self.color_cache.clear();
    }

    pub fn cache_size(&self) -> usize {
        self.color_cache.len()
    }
}

impl PaletteConverter for NearestColorConverter {
    fn convert_pixel(&self, rgb: Rgb<u8>) -> Rgb<u8> {
        // Use mutable reference to self through interior mutability pattern
        // For now, we'll compute without caching to avoid mutability issues
        self.find_nearest_color(rgb)
    }

    fn convert_image(&self, img: &DynamicImage) -> DynamicImage {
        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();

        log::info!("Converting {}x{} image to palette colors", width, height);

        let mut output_img: RgbImage = ImageBuffer::new(width, height);

        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let converted_pixel = self.convert_pixel(*pixel);
            output_img.put_pixel(x, y, converted_pixel);
        }

        DynamicImage::ImageRgb8(output_img)
    }
}

pub struct OptimizedConverter {
    converter: NearestColorConverter,
    pixel_area: (u32, u32),
    use_averaging: bool,
}

impl OptimizedConverter {
    pub fn new(
        palette: &Palette,
        selected_colors: &[String],
        pixel_area: Option<(u32, u32)>,
        use_averaging: bool,
    ) -> crate::Result<Self> {
        let converter = if selected_colors.is_empty() {
            NearestColorConverter::new(palette)?
        } else {
            NearestColorConverter::new_with_selected_colors(palette, selected_colors)?
        };

        let pixel_area = pixel_area.unwrap_or((1, 1));

        Ok(Self {
            converter,
            pixel_area,
            use_averaging,
        })
    }

    fn average_pixels(&self, img: &RgbImage, start_x: u32, start_y: u32) -> Rgb<u8> {
        let (width, height) = img.dimensions();
        let (area_w, area_h) = self.pixel_area;

        let mut total_r = 0u32;
        let mut total_g = 0u32;
        let mut total_b = 0u32;
        let mut pixel_count = 0u32;

        for y in start_y..std::cmp::min(start_y + area_h, height) {
            for x in start_x..std::cmp::min(start_x + area_w, width) {
                let pixel = img.get_pixel(x, y);
                total_r += u32::from(pixel[0]);
                total_g += u32::from(pixel[1]);
                total_b += u32::from(pixel[2]);
                pixel_count += 1;
            }
        }

        if pixel_count > 0 {
            Rgb([
                (total_r / pixel_count) as u8,
                (total_g / pixel_count) as u8,
                (total_b / pixel_count) as u8,
            ])
        } else {
            Rgb([0, 0, 0])
        }
    }
}

impl PaletteConverter for OptimizedConverter {
    fn convert_pixel(&self, rgb: Rgb<u8>) -> Rgb<u8> {
        self.converter.convert_pixel(rgb)
    }

    fn convert_image(&self, img: &DynamicImage) -> DynamicImage {
        if !self.use_averaging || self.pixel_area == (1, 1) {
            return self.converter.convert_image(img);
        }

        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        let (area_w, area_h) = self.pixel_area;

        log::info!(
            "Converting {}x{} image with {}x{} pixel area averaging",
            width,
            height,
            area_w,
            area_h
        );

        let mut output_img: RgbImage = ImageBuffer::new(width, height);

        for y in (0..height).step_by(area_h as usize) {
            for x in (0..width).step_by(area_w as usize) {
                let avg_color = self.average_pixels(&rgb_img, x, y);
                let converted_color = self.convert_pixel(avg_color);

                // Fill the entire area with the converted color
                for fill_y in y..std::cmp::min(y + area_h, height) {
                    for fill_x in x..std::cmp::min(x + area_w, width) {
                        output_img.put_pixel(fill_x, fill_y, converted_color);
                    }
                }
            }
        }

        DynamicImage::ImageRgb8(output_img)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, Palette};
    use std::path::PathBuf;

    fn create_test_palette() -> Palette {
        Palette {
            name: "test".to_string(),
            path: PathBuf::from("test"),
            colors: vec![
                Color {
                    name: "Red".to_string(),
                    hex: "#FF0000".to_string(),
                },
                Color {
                    name: "Green".to_string(),
                    hex: "#00FF00".to_string(),
                },
                Color {
                    name: "Blue".to_string(),
                    hex: "#0000FF".to_string(),
                },
            ],
        }
    }

    #[test]
    fn test_converter_creation() {
        let palette = create_test_palette();
        let converter = NearestColorConverter::new(&palette).unwrap();
        assert_eq!(converter.palette_colors.len(), 3);
    }

    #[test]
    fn test_convert_pixel() {
        let palette = create_test_palette();
        let converter = NearestColorConverter::new(&palette).unwrap();

        // Test red pixel should map to red
        let red_pixel = Rgb([255, 0, 0]);
        let result = converter.convert_pixel(red_pixel);
        assert_eq!(result, Rgb([255, 0, 0]));

        // Test pink pixel should map to red (nearest)
        let pink_pixel = Rgb([255, 100, 100]);
        let result = converter.convert_pixel(pink_pixel);
        assert_eq!(result, Rgb([255, 0, 0]));
    }

    #[test]
    fn test_selected_colors() {
        let palette = create_test_palette();
        let selected = vec!["Red".to_string(), "Blue".to_string()];
        let converter =
            NearestColorConverter::new_with_selected_colors(&palette, &selected).unwrap();

        assert_eq!(converter.palette_colors.len(), 2);

        // Green pixel should map to either red or blue (not green)
        let green_pixel = Rgb([0, 255, 0]);
        let result = converter.convert_pixel(green_pixel);
        assert!(result == Rgb([255, 0, 0]) || result == Rgb([0, 0, 255]));
    }

    #[test]
    fn test_empty_selected_colors() {
        let palette = create_test_palette();
        let selected = vec!["NonExistent".to_string()];
        let result = NearestColorConverter::new_with_selected_colors(&palette, &selected);
        assert!(result.is_err());
    }
}

use crate::{Config, Palette};
use image::DynamicImage;

use super::converter::{NearestColorConverter, OptimizedConverter, PaletteConverter};
use super::effects::{apply_gaussian_blur, BlurConfig};

pub struct ImageProcessor {
    config: Config,
}

impl ImageProcessor {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn process_image(
        &self,
        img: DynamicImage,
        palette: &Palette,
    ) -> crate::Result<DynamicImage> {
        log::info!("Starting image processing pipeline");

        // Step 1: Create the appropriate converter
        let converter = self.create_converter(palette)?;

        // Step 2: Convert image to palette colors
        let mut processed_img = converter.convert_image(&img);
        log::info!("Image converted to palette colors");

        // Step 3: Apply blur if enabled
        if self.config.enable_blur {
            processed_img = self.apply_blur(processed_img)?;
            log::info!("Gaussian blur applied");
        }

        // Step 4: Ensure output format matches input format
        processed_img = self.preserve_format(processed_img, &img);

        log::info!("Image processing pipeline completed");
        Ok(processed_img)
    }

    fn create_converter(&self, palette: &Palette) -> crate::Result<Box<dyn PaletteConverter>> {
        if self.config.pixels_area.is_some() || !self.config.disable_avg_pixels {
            // Use optimized converter for advanced features
            let converter = OptimizedConverter::new(
                palette,
                &self.config.colors,
                self.config.pixels_area,
                !self.config.disable_avg_pixels,
            )?;
            Ok(Box::new(converter))
        } else {
            // Use basic nearest color converter
            let converter = if self.config.colors.is_empty() {
                NearestColorConverter::new(palette)?
            } else {
                NearestColorConverter::new_with_selected_colors(palette, &self.config.colors)?
            };
            Ok(Box::new(converter))
        }
    }

    fn apply_blur(&self, img: DynamicImage) -> crate::Result<DynamicImage> {
        // Use enhanced blur configuration
        let blur_config = if let Some((w, h)) = self.config.pixels_area {
            // Use lighter blur for pixelated images
            if w > 2 || h > 2 {
                BlurConfig::light()
            } else {
                BlurConfig::moderate()
            }
        } else {
            BlurConfig::moderate()
        };

        let blurred = apply_gaussian_blur(&img, &blur_config);
        Ok(blurred)
    }

    fn preserve_format(
        &self,
        processed_img: DynamicImage,
        _original_img: &DynamicImage,
    ) -> DynamicImage {
        // For now, we'll keep it as RGB since palette conversion naturally produces RGB
        // In the future, we could add format-specific optimizations here
        processed_img
    }

    pub fn get_output_path(&self) -> &std::path::Path {
        &self.config.output_path
    }

    pub fn should_apply_blur(&self) -> bool {
        self.config.enable_blur
    }

    pub fn should_use_averaging(&self) -> bool {
        !self.config.disable_avg_pixels
    }

    pub fn get_pixel_area(&self) -> Option<(u32, u32)> {
        self.config.pixels_area
    }
}

pub struct ImagePipeline;

impl ImagePipeline {
    pub fn process_file(
        input_path: &std::path::Path,
        output_path: &std::path::Path,
        palette: &Palette,
        config: &Config,
    ) -> crate::Result<()> {
        log::info!(
            "Processing image: {} -> {}",
            input_path.display(),
            output_path.display()
        );

        // Load image
        let img = super::load_image(input_path)?;
        log::info!(
            "Loaded image: {}x{} pixels, format: {:?}",
            img.width(),
            img.height(),
            img.color()
        );

        // Create processor
        let processor = ImageProcessor::new(config.clone());

        // Process image
        let processed_img = processor.process_image(img, palette)?;

        // Save result
        super::save_image(&processed_img, output_path)?;

        log::info!("Image processing completed successfully");
        Ok(())
    }

    pub fn estimate_processing_time(img_width: u32, img_height: u32, config: &Config) -> f64 {
        let pixel_count = f64::from(img_width * img_height);

        // Base processing time (naive estimate)
        let mut time_estimate = pixel_count * 0.000001; // 1 microsecond per pixel base

        // Add time for averaging algorithm
        if !config.disable_avg_pixels {
            if let Some((area_w, area_h)) = config.pixels_area {
                let area_factor = f64::from(area_w * area_h);
                time_estimate *= 1.0 + (area_factor * 0.1);
            }
        }

        // Add time for blur
        if config.enable_blur {
            time_estimate *= 1.5; // 50% more time for blur
        }

        time_estimate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, Palette};
    use image::{ImageBuffer, Rgb};
    use std::path::PathBuf;

    fn create_test_config() -> Config {
        Config {
            input_path: Some(PathBuf::from("test_input.png")),
            output_path: PathBuf::from("test_output.png"),
            palette: "test".to_string(),
            colors: vec![],
            enable_blur: false,
            disable_avg_pixels: false,
            pixels_area: None,
            quiet_mode: false,
            benchmark: false,
        }
    }

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

    fn create_test_image() -> DynamicImage {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(10, 10, |x, _y| {
            if x < 5 {
                Rgb([255, 0, 0])
            } else {
                Rgb([0, 255, 0])
            }
        });
        DynamicImage::ImageRgb8(img)
    }

    #[test]
    fn test_processor_creation() {
        let config = create_test_config();
        let processor = ImageProcessor::new(config);
        assert!(!processor.should_apply_blur());
        assert!(processor.should_use_averaging());
    }

    #[test]
    fn test_process_basic_image() {
        let config = create_test_config();
        let processor = ImageProcessor::new(config);
        let palette = create_test_palette();
        let img = create_test_image();

        let result = processor.process_image(img, &palette);
        assert!(result.is_ok());

        let processed = result.unwrap();
        assert_eq!(processed.width(), 10);
        assert_eq!(processed.height(), 10);
    }

    #[test]
    fn test_blur_configuration() {
        let mut config = create_test_config();
        config.enable_blur = true;

        let processor = ImageProcessor::new(config);
        assert!(processor.should_apply_blur());
    }

    #[test]
    fn test_pixel_area_configuration() {
        let mut config = create_test_config();
        config.pixels_area = Some((2, 3));

        let processor = ImageProcessor::new(config);
        assert_eq!(processor.get_pixel_area(), Some((2, 3)));
    }

    #[test]
    fn test_processing_time_estimation() {
        let config = create_test_config();
        let time = ImagePipeline::estimate_processing_time(100, 100, &config);
        assert!(time > 0.0);

        let mut blur_config = create_test_config();
        blur_config.enable_blur = true;
        let blur_time = ImagePipeline::estimate_processing_time(100, 100, &blur_config);
        assert!(blur_time > time);
    }
}

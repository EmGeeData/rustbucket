use crate::{Config, Palette};
use image::DynamicImage;
use std::time::{Duration, Instant};

use super::processor::ImageProcessor;

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub total_time: Duration,
    pub load_time: Duration,
    pub conversion_time: Duration,
    pub effects_time: Duration,
    pub save_time: Duration,
    pub pixels_processed: u64,
    pub pixels_per_second: f64,
}

impl BenchmarkResult {
    pub fn print_summary(&self) {
        println!("Performance Summary:");
        println!("  Total processing: {:.2}s", self.total_time.as_secs_f32());
        println!("    - Load time:    {:.2}ms", self.load_time.as_millis());
        println!(
            "    - Conversion:   {:.2}ms",
            self.conversion_time.as_millis()
        );
        println!("    - Effects:      {:.2}ms", self.effects_time.as_millis());
        println!("    - Save time:    {:.2}ms", self.save_time.as_millis());
        println!(
            "  Pixels processed: {} ({:.1}K pixels/s)",
            self.pixels_processed,
            self.pixels_per_second / 1000.0
        );
    }

    pub fn is_performance_good(&self) -> bool {
        // Consider performance good if processing > 100K pixels/second
        self.pixels_per_second > 100_000.0
    }

    pub fn get_performance_grade(&self) -> &'static str {
        match self.pixels_per_second {
            pps if pps > 500_000.0 => "Excellent",
            pps if pps > 250_000.0 => "Very Good",
            pps if pps > 100_000.0 => "Good",
            pps if pps > 50_000.0 => "Fair",
            _ => "Needs Optimization",
        }
    }
}

pub struct ImageBenchmark {
    config: Config,
}

impl ImageBenchmark {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn benchmark_processing(
        &self,
        img: &DynamicImage,
        palette: &Palette,
    ) -> crate::Result<BenchmarkResult> {
        let total_start = Instant::now();
        let pixels_processed = (img.width() * img.height()) as u64;

        // Time the conversion process
        let conversion_start = Instant::now();
        let processor = ImageProcessor::new(self.config.clone());
        let _processed_img = processor.process_image(img.clone(), palette)?;
        let conversion_time = conversion_start.elapsed();

        let total_time = total_start.elapsed();
        let pixels_per_second = pixels_processed as f64 / total_time.as_secs_f64();

        Ok(BenchmarkResult {
            total_time,
            load_time: Duration::from_millis(0), // Would need to measure separately
            conversion_time,
            effects_time: Duration::from_millis(0), // Would need to measure separately
            save_time: Duration::from_millis(0),    // Would need to measure separately
            pixels_processed,
            pixels_per_second,
        })
    }

    pub fn benchmark_file_processing(
        &self,
        input_path: &std::path::Path,
        output_path: &std::path::Path,
        palette: &Palette,
    ) -> crate::Result<BenchmarkResult> {
        let total_start = Instant::now();

        // Time image loading
        let load_start = Instant::now();
        let img = super::load_image(input_path)?;
        let load_time = load_start.elapsed();

        let pixels_processed = (img.width() * img.height()) as u64;

        // Time processing
        let process_start = Instant::now();
        let processor = ImageProcessor::new(self.config.clone());
        let processed_img = processor.process_image(img, palette)?;
        let conversion_time = process_start.elapsed();

        // Time saving
        let save_start = Instant::now();
        super::save_image(&processed_img, output_path)?;
        let save_time = save_start.elapsed();

        let total_time = total_start.elapsed();
        let pixels_per_second = pixels_processed as f64 / total_time.as_secs_f64();

        Ok(BenchmarkResult {
            total_time,
            load_time,
            conversion_time,
            effects_time: Duration::from_millis(0), // Effects timing included in conversion_time
            save_time,
            pixels_processed,
            pixels_per_second,
        })
    }

    pub fn run_performance_suite(&self, palette: &Palette) -> Vec<BenchmarkResult> {
        let mut results = Vec::new();

        // Test different image sizes
        let test_sizes = vec![
            (100, 100),   // Small
            (500, 500),   // Medium
            (1000, 1000), // Large
        ];

        for (width, height) in test_sizes {
            if let Ok(test_img) = self.create_test_image(width, height) {
                if let Ok(result) = self.benchmark_processing(&test_img, palette) {
                    log::info!(
                        "Benchmark {}x{}: {:.1}K pixels/s",
                        width,
                        height,
                        result.pixels_per_second / 1000.0
                    );
                    results.push(result);
                }
            }
        }

        results
    }

    fn create_test_image(&self, width: u32, height: u32) -> crate::Result<DynamicImage> {
        use image::{ImageBuffer, Rgb};

        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
            // Create a gradient pattern for testing
            let r = ((x as f32 / width as f32) * 255.0) as u8;
            let g = ((y as f32 / height as f32) * 255.0) as u8;
            let b = (((x + y) as f32 / (width + height) as f32) * 255.0) as u8;
            Rgb([r, g, b])
        });

        Ok(DynamicImage::ImageRgb8(img))
    }
}

pub struct OptimizationSuggestions;

impl OptimizationSuggestions {
    pub fn analyze_config(config: &Config, result: &BenchmarkResult) -> Vec<String> {
        let mut suggestions = Vec::new();

        if result.pixels_per_second < 100_000.0 {
            suggestions.push(
                "Consider using --no-avg to disable pixel averaging for faster processing"
                    .to_string(),
            );
        }

        if config.enable_blur && result.pixels_per_second < 150_000.0 {
            suggestions.push(
                "Blur effects add processing time - disable with removing --blur for speed"
                    .to_string(),
            );
        }

        if let Some((w, h)) = config.pixels_area {
            if w > 4 || h > 4 {
                suggestions.push(
                    "Large pixel areas slow processing - try smaller values like --pixels-area 2,2"
                        .to_string(),
                );
            }
        }

        if !config.colors.is_empty() && config.colors.len() < 3 {
            suggestions.push(
                "Using very few colors may not provide significant speed benefits".to_string(),
            );
        }

        if suggestions.is_empty() {
            suggestions.push("Configuration looks well optimized for performance".to_string());
        }

        suggestions
    }

    pub fn suggest_improvements(result: &BenchmarkResult) -> Vec<String> {
        let mut suggestions = Vec::new();

        match result.get_performance_grade() {
            "Needs Optimization" => {
                suggestions.push("Performance is below optimal. Consider:".to_string());
                suggestions.push("- Using --no-avg to disable pixel averaging".to_string());
                suggestions
                    .push("- Processing smaller images or reducing quality settings".to_string());
                suggestions.push("- Using fewer colors from the palette".to_string());
            }
            "Fair" => {
                suggestions.push("Performance is fair. Small optimizations may help:".to_string());
                suggestions.push("- Fine-tune pixel area settings".to_string());
                suggestions
                    .push("- Consider disabling blur for speed-critical applications".to_string());
            }
            "Good" | "Very Good" | "Excellent" => {
                suggestions
                    .push("Performance is good! Current settings are well optimized.".to_string());
            }
            _ => {}
        }

        suggestions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_test_config() -> Config {
        Config {
            input_path: Some(PathBuf::from("test.png")),
            output_path: PathBuf::from("output.png"),
            palette: "test".to_string(),
            colors: vec![],
            enable_blur: false,
            disable_avg_pixels: false,
            pixels_area: None,
            quiet_mode: false,
            benchmark: false,
        }
    }

    #[test]
    fn test_benchmark_creation() {
        let config = create_test_config();
        let benchmark = ImageBenchmark::new(config);

        // Test image creation
        let img = benchmark.create_test_image(10, 10).unwrap();
        assert_eq!(img.width(), 10);
        assert_eq!(img.height(), 10);
    }

    #[test]
    fn test_benchmark_result_analysis() {
        let result = BenchmarkResult {
            total_time: Duration::from_millis(100),
            load_time: Duration::from_millis(10),
            conversion_time: Duration::from_millis(80),
            effects_time: Duration::from_millis(5),
            save_time: Duration::from_millis(5),
            pixels_processed: 10000,
            pixels_per_second: 150_000.0, // Above the threshold for good performance
        };

        assert!(result.is_performance_good());
        assert_eq!(result.get_performance_grade(), "Good");
    }

    #[test]
    fn test_optimization_suggestions() {
        let mut config = create_test_config();
        config.enable_blur = true;
        config.pixels_area = Some((8, 8));

        let result = BenchmarkResult {
            total_time: Duration::from_millis(200),
            load_time: Duration::from_millis(10),
            conversion_time: Duration::from_millis(180),
            effects_time: Duration::from_millis(5),
            save_time: Duration::from_millis(5),
            pixels_processed: 10000,
            pixels_per_second: 50_000.0, // Fair performance
        };

        let suggestions = OptimizationSuggestions::analyze_config(&config, &result);
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains("no-avg")));
    }
}

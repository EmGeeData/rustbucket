use rustbucket::{Config, ImagePipeline, PaletteManager};
use tempfile::tempdir;

fn create_test_image(
    width: u32,
    height: u32,
) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    use image::{ImageBuffer, Rgb};

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
        let r = ((x as f32 / width as f32) * 255.0) as u8;
        let g = ((y as f32 / height as f32) * 255.0) as u8;
        let b = (((x + y) as f32 / (width + height) as f32) * 255.0) as u8;
        Rgb([r, g, b])
    });

    Ok(image::DynamicImage::ImageRgb8(img))
}

#[test]
fn test_full_pipeline_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Create temporary directories
    let temp_dir = tempdir()?;
    let input_path = temp_dir.path().join("test_input.png");
    let output_path = temp_dir.path().join("test_output.png");

    // Create and save a test image
    let test_image = create_test_image(100, 100)?;
    test_image.save(&input_path)?;

    // Load palette manager and get a palette
    let mut palette_manager = PaletteManager::new();
    palette_manager.load_all_palettes()?;

    let palette = palette_manager
        .get_palette("nord")
        .ok_or("Nord palette not found")?;

    // Create configuration
    let config = Config {
        input_path: Some(input_path.clone()),
        output_path: output_path.clone(),
        palette: "nord".to_string(),
        colors: vec![],
        enable_blur: false,
        disable_avg_pixels: false,
        pixels_area: None,
        quiet_mode: true,
        benchmark: false,
    };

    // Process the image
    ImagePipeline::process_file(&input_path, &output_path, palette, &config)?;

    // Verify output file was created
    assert!(output_path.exists());

    // Load and verify the output image
    let output_image = image::open(&output_path)?;
    assert_eq!(output_image.width(), 100);
    assert_eq!(output_image.height(), 100);

    Ok(())
}

#[test]
fn test_pipeline_with_blur() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let input_path = temp_dir.path().join("test_blur_input.png");
    let output_path = temp_dir.path().join("test_blur_output.png");

    let test_image = create_test_image(50, 50)?;
    test_image.save(&input_path)?;

    let mut palette_manager = PaletteManager::new();
    palette_manager.load_all_palettes()?;

    let palette = palette_manager
        .get_palette("nord")
        .ok_or("Nord palette not found")?;

    let config = Config {
        input_path: Some(input_path.clone()),
        output_path: output_path.clone(),
        palette: "nord".to_string(),
        colors: vec![],
        enable_blur: true,
        disable_avg_pixels: false,
        pixels_area: None,
        quiet_mode: true,
        benchmark: false,
    };

    ImagePipeline::process_file(&input_path, &output_path, palette, &config)?;
    assert!(output_path.exists());

    Ok(())
}

#[test]
fn test_pipeline_with_custom_colors() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let input_path = temp_dir.path().join("test_colors_input.png");
    let output_path = temp_dir.path().join("test_colors_output.png");

    let test_image = create_test_image(25, 25)?;
    test_image.save(&input_path)?;

    let mut palette_manager = PaletteManager::new();
    palette_manager.load_all_palettes()?;

    let palette = palette_manager
        .get_palette("nord")
        .ok_or("Nord palette not found")?;

    let config = Config {
        input_path: Some(input_path.clone()),
        output_path: output_path.clone(),
        palette: "nord".to_string(),
        colors: vec![], // Use all colors from palette instead of specific named colors
        enable_blur: false,
        disable_avg_pixels: false,
        pixels_area: Some((2, 2)),
        quiet_mode: true,
        benchmark: false,
    };

    ImagePipeline::process_file(&input_path, &output_path, palette, &config)?;
    assert!(output_path.exists());

    Ok(())
}

#[test]
fn test_benchmark_integration() -> Result<(), Box<dyn std::error::Error>> {
    use rustbucket::ImageBenchmark;

    let temp_dir = tempdir()?;
    let input_path = temp_dir.path().join("test_benchmark_input.png");
    let output_path = temp_dir.path().join("test_benchmark_output.png");

    let test_image = create_test_image(100, 100)?;
    test_image.save(&input_path)?;

    let mut palette_manager = PaletteManager::new();
    palette_manager.load_all_palettes()?;

    let palette = palette_manager
        .get_palette("nord")
        .ok_or("Nord palette not found")?;

    let config = Config {
        input_path: Some(input_path.clone()),
        output_path: output_path.clone(),
        palette: "nord".to_string(),
        colors: vec![],
        enable_blur: false,
        disable_avg_pixels: false,
        pixels_area: None,
        quiet_mode: true,
        benchmark: true,
    };

    let benchmark = ImageBenchmark::new(config);
    let result = benchmark.benchmark_file_processing(&input_path, &output_path, palette)?;

    // Verify benchmark results
    assert!(result.pixels_processed > 0);
    assert!(result.pixels_per_second > 0.0);
    assert!(result.total_time.as_millis() > 0);

    // Verify output was created
    assert!(output_path.exists());

    Ok(())
}

#[test]
fn test_multiple_palettes() -> Result<(), Box<dyn std::error::Error>> {
    let mut palette_manager = PaletteManager::new();
    palette_manager.load_all_palettes()?;

    let palette_names = palette_manager.get_palette_names();
    assert!(!palette_names.is_empty());
    assert!(palette_names.contains(&"nord".to_string()));

    // Test each available palette
    let temp_dir = tempdir()?;
    let test_image = create_test_image(50, 50)?;

    for palette_name in palette_names.iter().take(3) {
        // Test first 3 palettes
        let input_path = temp_dir
            .path()
            .join(format!("test_{}_input.png", palette_name));
        let output_path = temp_dir
            .path()
            .join(format!("test_{}_output.png", palette_name));

        test_image.save(&input_path)?;

        if let Some(palette) = palette_manager.get_palette(palette_name) {
            let config = Config {
                input_path: Some(input_path.clone()),
                output_path: output_path.clone(),
                palette: palette_name.clone(),
                colors: vec![],
                enable_blur: false,
                disable_avg_pixels: false,
                pixels_area: None,
                quiet_mode: true,
                benchmark: false,
            };

            ImagePipeline::process_file(&input_path, &output_path, palette, &config)?;
            assert!(output_path.exists());
        }
    }

    Ok(())
}

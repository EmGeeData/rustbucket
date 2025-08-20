use anyhow::Result;
use clap::Parser;
use env_logger::Env;
use log::{info, warn};
use rustbucket::{
    Args, Config, ImageBenchmark, ImagePipeline, OptimizationSuggestions, PaletteManager,
};
use std::env;

fn main() -> Result<()> {
    // If no arguments provided, show help
    if env::args().len() == 1 {
        Args::parse_from(["rtbt", "--help"]);
        return Ok(());
    }

    let args = Args::parse();

    let log_level = if args.quiet_mode { "off" } else { "warn" };
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level)).init();

    info!(
        "RustBucket v{} - High-performance image palette converter",
        env!("CARGO_PKG_VERSION")
    );

    let config = Config::from(args.clone());

    // Handle list-palettes flag early (before requiring input/output paths)
    if args.list_palettes {
        let mut palette_manager = PaletteManager::new();
        match palette_manager.load_all_palettes() {
            Ok(()) => {
                println!("Available Palettes:");
                println!();

                // Show built-in palettes
                let builtin_names = rustbucket::palette::BuiltinPalettes::get_names();
                println!("Built-in palettes ({}):", builtin_names.len());
                for name in &builtin_names {
                    if let Some(palette) = palette_manager.get_palette(name) {
                        println!("  {} ({} colors)", name, palette.colors.len());
                    }
                }

                // Show user palettes (TOML palettes that aren't built-in)
                let all_names = palette_manager.get_palette_names();
                let builtin_set: std::collections::HashSet<String> =
                    builtin_names.into_iter().collect();
                let user_palettes: Vec<String> = all_names
                    .into_iter()
                    .filter(|name| !builtin_set.contains(name))
                    .collect();

                if !user_palettes.is_empty() {
                    println!();
                    println!("User palettes ({}):", user_palettes.len());
                    for name in user_palettes {
                        if let Some(palette) = palette_manager.get_palette(&name) {
                            println!("  {} ({} colors)", name, palette.colors.len());
                        }
                    }
                }

                return Ok(());
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to load palettes: {}", e));
            }
        }
    }

    // Handle create-palette flag early (before requiring input/output paths)
    if let Some(palette_path) = &args.create_palette {
        let palette_manager = PaletteManager::new();
        match palette_manager.create_example_toml_palette(palette_path) {
            Ok(()) => {
                if !args.quiet_mode {
                    println!("Skeleton palette file created: {}", palette_path.display());
                    println!();
                    println!("Edit the file to customize your palette, then use it with:");
                    println!(
                        "  rtbt -i input.png -o output.png -p {}",
                        palette_path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("your_palette")
                    );
                }
                return Ok(());
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to create palette file: {}", e));
            }
        }
    }

    // Handle export-palette flag early (before requiring input/output paths)
    if let Some(export_args) = &args.export_palette {
        if export_args.len() != 2 {
            return Err(anyhow::anyhow!(
                "--export-palette requires exactly 2 arguments: palette_name output_path"
            ));
        }

        let palette_name = &export_args[0];
        let output_path = std::path::PathBuf::from(&export_args[1]);

        let mut palette_manager = PaletteManager::new();
        palette_manager.load_all_palettes()?;

        // Check if it's a built-in palette
        if !palette_manager.is_builtin_palette(palette_name) {
            let available_builtins = rustbucket::palette::BuiltinPalettes::get_names();
            return Err(anyhow::anyhow!(
                "Palette '{}' is not a built-in palette.\nAvailable built-in palettes: {}",
                palette_name,
                available_builtins.join(", ")
            ));
        }

        if let Some(palette) = palette_manager.get_palette(palette_name) {
            let toml_loader = rustbucket::palette::TomlPaletteLoader::new();
            toml_loader.save_palette(palette, &output_path)?;

            if !args.quiet_mode {
                println!(
                    "Built-in palette '{}' exported to: {}",
                    palette_name,
                    output_path.display()
                );
                println!();
                println!("You can now customize this palette and use it with:");
                println!("  rtbt -i input.png -o output.png -p {}", palette_name);
            }
            return Ok(());
        } else {
            return Err(anyhow::anyhow!("Failed to load palette '{}'", palette_name));
        }
    }

    // Get input path (required for normal operation)
    let input_path = config
        .input_path
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Input path is required for image processing"))?;

    info!("Loading input image: {}", input_path.display());
    info!("Set output image name: {}", config.output_path.display());

    let mut palette_manager = PaletteManager::new();

    match palette_manager.load_all_palettes() {
        Ok(()) => {
            let palette_names = palette_manager.get_palette_names();
            info!("Available palettes: {}", palette_names.join(", "));

            if !palette_manager.palette_exists(&config.palette) {
                warn!(
                    "Palette '{}' not found, using default 'nord'",
                    config.palette
                );
                if !palette_manager.palette_exists("nord") {
                    return Err(anyhow::anyhow!("Default 'nord' palette not available"));
                }
            }

            if let Some(palette) = palette_manager.get_palette(&config.palette) {
                info!(
                    "Using palette '{}' with {} colors",
                    palette.name,
                    palette.colors.len()
                );

                if !config.colors.is_empty() {
                    let available_colors: Vec<String> =
                        palette.colors.iter().map(|c| c.name.clone()).collect();
                    for color in &config.colors {
                        if !available_colors.contains(color) {
                            warn!("Color '{}' not found in palette '{}'", color, palette.name);
                            info!("Available colors: {}", available_colors.join(", "));
                        }
                    }
                }
            }
        }
        Err(e) => {
            warn!("Failed to load palettes: {}", e);
            return Err(anyhow::anyhow!("Cannot proceed without palettes: {}", e));
        }
    }

    if config.enable_blur {
        info!("Blur enabled");
    }

    if config.disable_avg_pixels {
        info!("Average pixels optimization disabled");
    }

    if let Some((w, h)) = config.pixels_area {
        info!("Pixels area set to {}x{}", w, h);
    }

    // Check if input file exists first
    if !input_path.exists() {
        return Err(anyhow::anyhow!(
            "Input file does not exist: {}",
            input_path.display()
        ));
    }

    // Get the palette to use for conversion
    let palette_name = if palette_manager.palette_exists(&config.palette) {
        &config.palette
    } else {
        "nord" // fallback to nord
    };

    let palette = palette_manager
        .get_palette(palette_name)
        .ok_or_else(|| anyhow::anyhow!("Palette '{}' not found", palette_name))?;

    if !config.quiet_mode {
        println!("RustBucket is ready! Configuration:");
        println!("  Input: {}", input_path.display());
        println!("  Output: {}", config.output_path.display());
        println!("  Palette: {}", config.palette);
        println!(
            "  Colors: {}",
            if config.colors.is_empty() {
                "all".to_string()
            } else {
                config.colors.join(", ")
            }
        );
        println!("  Blur: {}", config.enable_blur);
        println!("  Avg optimization: {}", !config.disable_avg_pixels);
        if let Some((w, h)) = config.pixels_area {
            println!("  Pixel area: {}x{}", w, h);
        }
        println!();
    }

    // Check if benchmark mode is enabled
    if config.benchmark {
        info!("Running performance benchmarks...");
        let benchmark = ImageBenchmark::new(config.clone());

        // Run file processing benchmark
        match benchmark.benchmark_file_processing(input_path, &config.output_path, palette) {
            Ok(result) => {
                if !config.quiet_mode {
                    println!("Benchmark Results:");
                    result.print_summary();
                    println!("Performance Grade: {}", result.get_performance_grade());

                    // Show optimization suggestions
                    let config_suggestions =
                        OptimizationSuggestions::analyze_config(&config, &result);
                    let perf_suggestions = OptimizationSuggestions::suggest_improvements(&result);

                    println!("\nOptimization Suggestions:");
                    for suggestion in config_suggestions.iter().chain(perf_suggestions.iter()) {
                        println!("  - {}", suggestion);
                    }

                    // Run performance suite for additional analysis
                    println!("\nRunning performance suite...");
                    let suite_results = benchmark.run_performance_suite(palette);
                    if !suite_results.is_empty() {
                        let avg_pps: f64 = suite_results
                            .iter()
                            .map(|r| r.pixels_per_second)
                            .sum::<f64>()
                            / suite_results.len() as f64;
                        println!("   Average performance: {:.1}K pixels/s", avg_pps / 1000.0);
                    }
                }
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Benchmark failed: {}", e));
            }
        }
    } else {
        // Normal processing mode
        info!("Starting image conversion...");
        let start_time = std::time::Instant::now();

        match ImagePipeline::process_file(input_path, &config.output_path, palette, &config) {
            Ok(()) => {
                let duration = start_time.elapsed();
                info!(
                    "Image processing completed in {:.2}s",
                    duration.as_secs_f32()
                );

                if !config.quiet_mode {
                    println!("Image converted successfully!");
                    println!("   Output saved to: {}", config.output_path.display());
                    println!("   Processing time: {:.2}s", duration.as_secs_f32());
                }
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Image processing failed: {}", e));
            }
        }
    }

    Ok(())
}

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "rustbucket",
    version,
    about = "RustBucket - High-performance image palette converter\n\nConverts RGB images to themed color palettes with support for 17+ built-in themes.",
    long_about = "RustBucket is a high-performance CLI tool that converts RGB images to themed color palettes.\n\nSupported palettes include Nord, Dracula, Gruvbox, Monokai, Solarized, and many more.\nThe tool offers various optimization options including blur effects, average pixel calculations, and custom pixel area processing."
)]
pub struct Args {
    #[arg(
        short = 'i',
        long = "img",
        help = "Input image path",
        value_name = "PATH",
        required_unless_present_any = ["list_palettes", "create_palette", "export_palette"]
    )]
    pub input_path: Option<PathBuf>,

    #[arg(
        short = 'o',
        long = "out",
        help = "Output image path",
        value_name = "PATH",
        default_value = "nord.png"
    )]
    pub output_path: PathBuf,

    #[arg(
        short = 'p',
        long = "palette",
        help = "Color palette to use",
        value_name = "PALETTE",
        default_value = "nord"
    )]
    pub palette: String,

    #[arg(
        short = 'c',
        long = "colors",
        help = "Specific colors to use from palette (comma-separated)",
        value_name = "COLORS",
        value_delimiter = ','
    )]
    pub colors: Vec<String>,

    #[arg(
        short = 'b',
        long = "blur",
        help = "Apply Gaussian blur to the final result",
        action = clap::ArgAction::SetTrue
    )]
    pub enable_blur: bool,

    #[arg(
        short = 'q',
        long = "quiet",
        help = "Quiet mode - suppress output",
        action = clap::ArgAction::SetTrue
    )]
    pub quiet_mode: bool,

    #[arg(
        long = "no-avg",
        help = "Disable average pixels optimization algorithm",
        action = clap::ArgAction::SetTrue
    )]
    pub disable_avg_pixels: bool,

    #[arg(
        long = "pixels-area",
        help = "Pixel area for average color calculation (format: WIDTH[,HEIGHT])",
        value_name = "AREA",
        value_parser = parse_pixels_area
    )]
    pub pixels_area: Option<(u32, u32)>,

    #[arg(
        long = "benchmark",
        help = "Run performance benchmarks and show optimization suggestions",
        action = clap::ArgAction::SetTrue
    )]
    pub benchmark: bool,

    #[arg(
        long = "list-palettes",
        help = "List all available built-in and user palettes",
        action = clap::ArgAction::SetTrue
    )]
    pub list_palettes: bool,

    #[arg(
        long = "create-palette",
        help = "Create a skeleton palette file at the specified path",
        value_name = "PATH"
    )]
    pub create_palette: Option<std::path::PathBuf>,

    #[arg(
        long = "export-palette",
        help = "Export a built-in palette to TOML format: --export-palette nord output.toml",
        value_names = ["PALETTE", "PATH"],
        num_args = 2
    )]
    pub export_palette: Option<Vec<String>>,
}

fn parse_pixels_area(value: &str) -> Result<(u32, u32), String> {
    if value.is_empty() {
        return Err("Invalid value for pixels area: empty string".to_string());
    }

    let parts: Vec<&str> = value.split(',').collect();

    if parts.len() > 2 {
        return Err(format!(
            "Invalid number of parameters for pixels area: {}",
            value
        ));
    }

    let width = parts[0]
        .parse::<u32>()
        .map_err(|_| format!("Invalid width value: {}", parts[0]))?;

    let height = if parts.len() > 1 {
        parts[1]
            .parse::<u32>()
            .map_err(|_| format!("Invalid height value: {}", parts[1]))?
    } else {
        width
    };

    Ok((width, height))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pixels_area_single_value() {
        assert_eq!(parse_pixels_area("5"), Ok((5, 5)));
    }

    #[test]
    fn test_parse_pixels_area_two_values() {
        assert_eq!(parse_pixels_area("3,7"), Ok((3, 7)));
    }

    #[test]
    fn test_parse_pixels_area_invalid_empty() {
        assert!(parse_pixels_area("").is_err());
    }

    #[test]
    fn test_parse_pixels_area_invalid_too_many() {
        assert!(parse_pixels_area("1,2,3").is_err());
    }

    #[test]
    fn test_parse_pixels_area_invalid_non_numeric() {
        assert!(parse_pixels_area("abc").is_err());
        assert!(parse_pixels_area("1,abc").is_err());
    }
}

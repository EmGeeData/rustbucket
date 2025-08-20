use crate::{Color, Palette, RustBucketError};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// TOML palette configuration structure
#[derive(Debug, Deserialize, Serialize)]
pub struct TomlPalette {
    pub name: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub colors: Vec<TomlColor>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TomlColor {
    pub name: String,
    pub hex: String,
    pub description: Option<String>,
}

impl From<TomlPalette> for Palette {
    fn from(toml_palette: TomlPalette) -> Self {
        Palette {
            name: toml_palette.name.clone(),
            path: PathBuf::from(format!("user://{}", toml_palette.name)),
            colors: toml_palette
                .colors
                .into_iter()
                .map(|c| Color {
                    name: c.name,
                    hex: c.hex,
                })
                .collect(),
        }
    }
}

impl From<&Palette> for TomlPalette {
    fn from(palette: &Palette) -> Self {
        TomlPalette {
            name: palette.name.clone(),
            description: None,
            author: None,
            colors: palette
                .colors
                .iter()
                .map(|c| TomlColor {
                    name: c.name.clone(),
                    hex: c.hex.clone(),
                    description: None,
                })
                .collect(),
        }
    }
}

pub struct TomlPaletteLoader {
    search_paths: Vec<PathBuf>,
}

impl TomlPaletteLoader {
    pub fn new() -> Self {
        Self {
            search_paths: Self::get_default_search_paths(),
        }
    }

    pub fn with_paths(paths: Vec<PathBuf>) -> Self {
        Self {
            search_paths: paths,
        }
    }

    pub fn with_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            search_paths: vec![path.as_ref().to_path_buf()],
        }
    }

    /// Get default search paths for palettes
    fn get_default_search_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();

        // 1. Environment variable override
        if let Ok(custom_path) = env::var("RTBT_PALETTE_DIR") {
            paths.push(PathBuf::from(custom_path));
        }

        // 2. User config directory (~/.config/rtbt/palettes/)
        if let Some(config_dir) = Self::get_user_config_dir() {
            paths.push(config_dir.join("rtbt").join("palettes"));
        }

        paths
    }

    /// Get user config directory
    fn get_user_config_dir() -> Option<PathBuf> {
        // Try XDG_CONFIG_HOME first, then fall back to HOME/.config
        env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
    }

    /// Load all TOML palettes from all search paths
    pub fn load_palettes(&self) -> crate::Result<Vec<Palette>> {
        let mut palettes = Vec::new();
        let mut found_directories = Vec::new();

        // Search through all configured paths
        for search_path in &self.search_paths {
            if !search_path.exists() {
                log::debug!("Palette directory not found: {}", search_path.display());
                continue;
            }

            log::debug!("Searching for palettes in: {}", search_path.display());
            found_directories.push(search_path.clone());

            let entries = fs::read_dir(search_path).map_err(|e| {
                RustBucketError::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Failed to read palette directory {}: {}",
                        search_path.display(),
                        e
                    ),
                ))
            })?;

            for entry in entries {
                let entry = entry.map_err(RustBucketError::IoError)?;
                let path = entry.path();

                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml") {
                    match self.load_toml_palette(&path) {
                        Ok(palette) => {
                            // Check if we already have a palette with this name
                            if palettes.iter().any(|p: &Palette| p.name == palette.name) {
                                log::debug!(
                                    "Skipping duplicate palette '{}' from {}",
                                    palette.name,
                                    path.display()
                                );
                            } else {
                                log::debug!(
                                    "Loaded palette '{}' from {}",
                                    palette.name,
                                    path.display()
                                );
                                palettes.push(palette);
                            }
                        }
                        Err(e) => log::warn!("Failed to load palette {}: {}", path.display(), e),
                    }
                }
            }
        }

        if found_directories.is_empty() {
            log::info!("No palette directories found in search paths");
        } else {
            log::info!(
                "Loaded {} TOML palettes from {} directories",
                palettes.len(),
                found_directories.len()
            );
        }

        Ok(palettes)
    }

    /// Load a single TOML palette file
    pub fn load_toml_palette<P: AsRef<Path>>(&self, path: P) -> crate::Result<Palette> {
        let content = fs::read_to_string(path.as_ref()).map_err(RustBucketError::IoError)?;

        let toml_palette: TomlPalette = toml::from_str(&content).map_err(|e| {
            RustBucketError::PaletteParseError(format!(
                "Failed to parse TOML palette {}: {}",
                path.as_ref().display(),
                e
            ))
        })?;

        // Validate colors
        for color in &toml_palette.colors {
            super::parser::parse_hex_color(&color.hex, color.name.clone()).map_err(|e| {
                RustBucketError::PaletteParseError(format!(
                    "Invalid color '{}' in palette '{}': {}",
                    color.hex, toml_palette.name, e
                ))
            })?;
        }

        Ok(toml_palette.into())
    }

    /// Save a palette as TOML
    pub fn save_palette<P: AsRef<Path>>(&self, palette: &Palette, path: P) -> crate::Result<()> {
        let toml_palette = TomlPalette::from(palette);
        let toml_content = toml::to_string_pretty(&toml_palette).map_err(|e| {
            RustBucketError::PaletteParseError(format!("Failed to serialize palette: {}", e))
        })?;

        fs::write(path.as_ref(), toml_content).map_err(RustBucketError::IoError)?;
        Ok(())
    }

    /// Create an example TOML palette file
    pub fn create_example_palette<P: AsRef<Path>>(&self, path: P) -> crate::Result<()> {
        let example = TomlPalette {
            name: "example".to_string(),
            description: Some("An example custom palette for image tinting".to_string()),
            author: Some("Your Name".to_string()),
            colors: vec![
                TomlColor {
                    name: "dark_blue".to_string(),
                    hex: "#2E3440".to_string(),
                    description: Some("Deep blue-grey tone".to_string()),
                },
                TomlColor {
                    name: "light_blue".to_string(),
                    hex: "#88C0D0".to_string(),
                    description: Some("Soft cyan-blue".to_string()),
                },
                TomlColor {
                    name: "green".to_string(),
                    hex: "#A3BE8C".to_string(),
                    description: Some("Muted green".to_string()),
                },
                TomlColor {
                    name: "yellow".to_string(),
                    hex: "#EBCB8B".to_string(),
                    description: Some("Warm yellow".to_string()),
                },
                TomlColor {
                    name: "orange".to_string(),
                    hex: "#D08770".to_string(),
                    description: Some("Soft orange".to_string()),
                },
                TomlColor {
                    name: "red".to_string(),
                    hex: "#BF616A".to_string(),
                    description: Some("Muted red".to_string()),
                },
                TomlColor {
                    name: "purple".to_string(),
                    hex: "#B48EAD".to_string(),
                    description: Some("Soft purple".to_string()),
                },
                TomlColor {
                    name: "white".to_string(),
                    hex: "#ECEFF4".to_string(),
                    description: Some("Light neutral".to_string()),
                },
            ],
        };

        let toml_content = toml::to_string_pretty(&example).map_err(|e| {
            RustBucketError::PaletteParseError(format!("Failed to create example palette: {}", e))
        })?;

        fs::write(path.as_ref(), toml_content).map_err(RustBucketError::IoError)?;
        log::info!("Created example palette at: {}", path.as_ref().display());
        Ok(())
    }
}

impl Default for TomlPaletteLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_toml_palette_conversion() {
        let toml_palette = TomlPalette {
            name: "test".to_string(),
            description: Some("Test palette".to_string()),
            author: Some("Test Author".to_string()),
            colors: vec![
                TomlColor {
                    name: "red".to_string(),
                    hex: "#FF0000".to_string(),
                    description: Some("Red color".to_string()),
                },
                TomlColor {
                    name: "blue".to_string(),
                    hex: "#0000FF".to_string(),
                    description: None,
                },
            ],
        };

        let palette: Palette = toml_palette.into();
        assert_eq!(palette.name, "test");
        assert_eq!(palette.colors.len(), 2);
        assert_eq!(palette.colors[0].name, "red");
        assert_eq!(palette.colors[0].hex, "#FF0000");
    }

    #[test]
    fn test_palette_to_toml_conversion() {
        let palette = Palette {
            name: "test".to_string(),
            path: PathBuf::from("test.toml"),
            colors: vec![
                Color {
                    name: "red".to_string(),
                    hex: "#FF0000".to_string(),
                },
                Color {
                    name: "blue".to_string(),
                    hex: "#0000FF".to_string(),
                },
            ],
        };

        let toml_palette = TomlPalette::from(&palette);
        assert_eq!(toml_palette.name, "test");
        assert_eq!(toml_palette.colors.len(), 2);
        assert_eq!(toml_palette.colors[0].name, "red");
        assert_eq!(toml_palette.colors[0].hex, "#FF0000");
    }

    #[test]
    fn test_load_empty_directory() {
        let temp_dir = tempdir().unwrap();
        let loader = TomlPaletteLoader::with_path(temp_dir.path());
        let palettes = loader.load_palettes().unwrap();
        assert!(palettes.is_empty());
    }

    #[test]
    fn test_multi_path_loading() {
        let temp_dir1 = tempdir().unwrap();
        let temp_dir2 = tempdir().unwrap();

        // Create palette in first directory
        let palette1_path = temp_dir1.path().join("theme1.toml");
        let toml_content1 = r##"
name = "theme1"
description = "First theme"

[[colors]]
name = "red"
hex = "#FF0000"
"##;
        fs::write(&palette1_path, toml_content1).unwrap();

        // Create palette in second directory
        let palette2_path = temp_dir2.path().join("theme2.toml");
        let toml_content2 = r##"
name = "theme2"
description = "Second theme"

[[colors]]
name = "blue"
hex = "#0000FF"
"##;
        fs::write(&palette2_path, toml_content2).unwrap();

        // Test multi-path loading
        let paths = vec![
            temp_dir1.path().to_path_buf(),
            temp_dir2.path().to_path_buf(),
        ];
        let loader = TomlPaletteLoader::with_paths(paths);
        let palettes = loader.load_palettes().unwrap();

        assert_eq!(palettes.len(), 2);
        assert!(palettes.iter().any(|p| p.name == "theme1"));
        assert!(palettes.iter().any(|p| p.name == "theme2"));
    }

    #[test]
    fn test_duplicate_palette_handling() {
        let temp_dir1 = tempdir().unwrap();
        let temp_dir2 = tempdir().unwrap();

        // Create same palette name in both directories
        let palette1_path = temp_dir1.path().join("duplicate.toml");
        let toml_content1 = r##"
name = "duplicate"
description = "First version"

[[colors]]
name = "red"
hex = "#FF0000"
"##;
        fs::write(&palette1_path, toml_content1).unwrap();

        let palette2_path = temp_dir2.path().join("duplicate.toml");
        let toml_content2 = r##"
name = "duplicate"
description = "Second version"

[[colors]]
name = "blue"
hex = "#0000FF"
"##;
        fs::write(&palette2_path, toml_content2).unwrap();

        // Test that first one wins (no duplicates)
        let paths = vec![
            temp_dir1.path().to_path_buf(),
            temp_dir2.path().to_path_buf(),
        ];
        let loader = TomlPaletteLoader::with_paths(paths);
        let palettes = loader.load_palettes().unwrap();

        assert_eq!(palettes.len(), 1);
        assert_eq!(palettes[0].name, "duplicate");
        // Should be the first one (red color)
        assert_eq!(palettes[0].colors[0].hex, "#FF0000");
    }

    #[test]
    fn test_create_and_load_example_palette() {
        let temp_dir = tempdir().unwrap();
        let palette_path = temp_dir.path().join("example.toml");

        let loader = TomlPaletteLoader::with_path(temp_dir.path());
        loader.create_example_palette(&palette_path).unwrap();

        assert!(palette_path.exists());

        let palette = loader.load_toml_palette(&palette_path).unwrap();
        assert_eq!(palette.name, "example");
        assert_eq!(palette.colors.len(), 8);
        assert_eq!(palette.colors[0].name, "dark_blue");
        assert_eq!(palette.colors[0].hex, "#2E3440");
    }

    #[test]
    fn test_save_and_load_palette() {
        let temp_dir = tempdir().unwrap();
        let palette_path = temp_dir.path().join("test.toml");

        let original_palette = Palette {
            name: "test_save".to_string(),
            path: PathBuf::from("test"),
            colors: vec![
                Color {
                    name: "primary".to_string(),
                    hex: "#FF5733".to_string(),
                },
                Color {
                    name: "secondary".to_string(),
                    hex: "#33FF57".to_string(),
                },
            ],
        };

        let loader = TomlPaletteLoader::with_path(temp_dir.path());
        loader
            .save_palette(&original_palette, &palette_path)
            .unwrap();

        let loaded_palette = loader.load_toml_palette(&palette_path).unwrap();
        assert_eq!(loaded_palette.name, original_palette.name);
        assert_eq!(loaded_palette.colors.len(), original_palette.colors.len());
        assert_eq!(loaded_palette.colors[0].hex, original_palette.colors[0].hex);
    }
}

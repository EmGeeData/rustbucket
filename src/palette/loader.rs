use crate::{Color, Palette, RustBucketError};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::parser::parse_hex_color;

pub struct PaletteLoader {
    palette_root: PathBuf,
}

impl PaletteLoader {
    pub fn new() -> Self {
        Self {
            palette_root: PathBuf::from("palettes"),
        }
    }

    pub fn with_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            palette_root: path.as_ref().to_path_buf(),
        }
    }

    pub fn load_palettes(&self) -> crate::Result<HashMap<String, Palette>> {
        let mut palettes = HashMap::new();

        if !self.palette_root.exists() {
            return Err(RustBucketError::PaletteNotFound(format!(
                "Palette directory not found: {}",
                self.palette_root.display()
            )));
        }

        let entries = fs::read_dir(&self.palette_root).map_err(RustBucketError::IoError)?;

        for entry in entries {
            let entry = entry.map_err(RustBucketError::IoError)?;
            let path = entry.path();

            if path.is_dir() {
                let palette_name = path.file_name().and_then(|n| n.to_str()).ok_or_else(|| {
                    RustBucketError::PaletteNotFound(format!(
                        "Invalid palette directory name: {}",
                        path.display()
                    ))
                })?;

                match self.load_palette(&path, palette_name) {
                    Ok(palette) => {
                        palettes.insert(palette_name.to_lowercase(), palette);
                    }
                    Err(e) => {
                        log::warn!("Failed to load palette '{}': {}", palette_name, e);
                    }
                }
            }
        }

        if palettes.is_empty() {
            return Err(RustBucketError::PaletteNotFound(
                "No valid palettes found".to_string(),
            ));
        }

        log::info!("Loaded {} palettes", palettes.len());
        Ok(palettes)
    }

    fn load_palette(&self, palette_dir: &Path, name: &str) -> crate::Result<Palette> {
        let mut colors = Vec::new();

        let entries = fs::read_dir(palette_dir).map_err(RustBucketError::IoError)?;

        for entry in entries {
            let entry = entry.map_err(RustBucketError::IoError)?;
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "txt") {
                let color_name = path.file_stem().and_then(|n| n.to_str()).ok_or_else(|| {
                    RustBucketError::InvalidColor(format!(
                        "Invalid color file name: {}",
                        path.display()
                    ))
                })?;

                match self.load_color_file(&path, color_name) {
                    Ok(mut file_colors) => {
                        colors.append(&mut file_colors);
                    }
                    Err(e) => {
                        log::warn!("Failed to load color file '{}': {}", path.display(), e);
                    }
                }
            }
        }

        if colors.is_empty() {
            return Err(RustBucketError::PaletteNotFound(format!(
                "No valid colors found in palette: {}",
                name
            )));
        }

        Ok(Palette {
            name: name.to_string(),
            path: palette_dir.to_path_buf(),
            colors,
        })
    }

    fn load_color_file(&self, file_path: &Path, color_name: &str) -> crate::Result<Vec<Color>> {
        let content = fs::read_to_string(file_path).map_err(RustBucketError::IoError)?;

        let mut colors = Vec::new();
        let mut color_index = 0;

        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();

            if line.is_empty() || line.starts_with("//") || line.starts_with("#") && line.len() == 1
            {
                continue;
            }

            let final_color_name = if colors.is_empty() {
                color_name.to_string()
            } else {
                format!("{}{}", color_name, color_index)
            };

            match parse_hex_color(line, final_color_name) {
                Ok(color) => {
                    colors.push(color);
                    color_index += 1;
                }
                Err(e) => {
                    log::warn!(
                        "Invalid color on line {} in file {}: {}",
                        line_num + 1,
                        file_path.display(),
                        e
                    );
                }
            }
        }

        if colors.is_empty() {
            return Err(RustBucketError::InvalidColor(format!(
                "No valid colors found in file: {}",
                file_path.display()
            )));
        }

        Ok(colors)
    }
}

impl Default for PaletteLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_palette_structure() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let palettes_dir = temp_dir.path();

        let nord_dir = palettes_dir.join("Nord");
        fs::create_dir_all(&nord_dir).unwrap();

        fs::write(nord_dir.join("Aurora.txt"), "#BF616A\n#D08770\n#EBCB8B").unwrap();
        fs::write(nord_dir.join("Frost.txt"), "#8FBCBB\n#88C0D0").unwrap();

        let dracula_dir = palettes_dir.join("Dracula");
        fs::create_dir_all(&dracula_dir).unwrap();

        fs::write(dracula_dir.join("Blue.txt"), "#6272a4\n#8be9fd").unwrap();

        temp_dir
    }

    #[test]
    fn test_load_palettes() {
        let temp_dir = create_test_palette_structure();
        let loader = PaletteLoader::with_path(temp_dir.path());

        let palettes = loader.load_palettes().unwrap();

        assert_eq!(palettes.len(), 2);
        assert!(palettes.contains_key("nord"));
        assert!(palettes.contains_key("dracula"));
    }

    #[test]
    fn test_load_nord_palette() {
        let temp_dir = create_test_palette_structure();
        let loader = PaletteLoader::with_path(temp_dir.path());

        let palettes = loader.load_palettes().unwrap();
        let nord = palettes.get("nord").unwrap();

        assert_eq!(nord.name, "Nord");
        assert_eq!(nord.colors.len(), 5); // 3 Aurora + 2 Frost

        // Check that we have colors from both files (order may vary)
        let color_names: Vec<&String> = nord.colors.iter().map(|c| &c.name).collect();
        let has_aurora = color_names.iter().any(|name| name.contains("Aurora"));
        let has_frost = color_names.iter().any(|name| name.contains("Frost"));

        assert!(has_aurora, "Should have Aurora colors");
        assert!(has_frost, "Should have Frost colors");

        // Check that we have the expected hex values
        let hex_values: Vec<&String> = nord.colors.iter().map(|c| &c.hex).collect();
        assert!(hex_values.contains(&&"#BF616A".to_string()));
        assert!(hex_values.contains(&&"#8FBCBB".to_string()));
    }

    #[test]
    fn test_load_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let empty_dir = temp_dir.path().join("empty");
        fs::create_dir_all(&empty_dir).unwrap();

        let loader = PaletteLoader::with_path(&empty_dir);
        assert!(loader.load_palettes().is_err());
    }

    #[test]
    fn test_nonexistent_directory() {
        let loader = PaletteLoader::with_path("/nonexistent/path");
        assert!(loader.load_palettes().is_err());
    }
}

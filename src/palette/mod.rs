mod builtin;
mod loader;
mod parser;
mod toml_loader;

pub use builtin::BuiltinPalettes;
pub use loader::PaletteLoader;
pub use parser::{hex_to_rgb, parse_hex_color, rgb_to_hex};
pub use toml_loader::{TomlPalette, TomlPaletteLoader};

use crate::Palette;
use std::collections::HashMap;

pub struct PaletteManager {
    palettes: HashMap<String, Palette>,
}

impl PaletteManager {
    pub fn new() -> Self {
        Self {
            palettes: HashMap::new(),
        }
    }

    /// Load all palettes (built-in and TOML)
    pub fn load_all_palettes(&mut self) -> crate::Result<()> {
        // Start with built-in palettes
        let builtin_palettes = BuiltinPalettes::get_all();
        for palette in builtin_palettes {
            self.palettes.insert(palette.name.to_lowercase(), palette);
        }

        // Load TOML palettes (user-defined)
        let toml_loader = TomlPaletteLoader::new();
        let toml_palettes = toml_loader.load_palettes()?;
        for palette in toml_palettes {
            // TOML palettes can override built-in ones
            self.palettes.insert(palette.name.to_lowercase(), palette);
        }

        log::info!("Loaded {} total palettes", self.palettes.len());
        Ok(())
    }

    pub fn get_palette(&self, name: &str) -> Option<&Palette> {
        self.palettes.get(&name.to_lowercase())
    }

    pub fn get_palette_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.palettes.keys().cloned().collect();
        names.sort();
        names
    }

    pub fn palette_exists(&self, name: &str) -> bool {
        self.palettes.contains_key(&name.to_lowercase())
    }

    /// Get built-in palette names
    pub fn get_builtin_palette_names(&self) -> Vec<String> {
        BuiltinPalettes::get_names()
    }

    /// Check if a palette is built-in
    pub fn is_builtin_palette(&self, name: &str) -> bool {
        BuiltinPalettes::get_palette(name).is_some()
    }

    /// Create an example TOML palette file
    pub fn create_example_toml_palette<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> crate::Result<()> {
        let toml_loader = TomlPaletteLoader::new();
        toml_loader.create_example_palette(path)
    }
}

impl Default for PaletteManager {
    fn default() -> Self {
        Self::new()
    }
}

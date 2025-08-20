use crate::{Color, Palette};
use std::path::PathBuf;

/// Built-in palette definitions
pub struct BuiltinPalettes;

impl BuiltinPalettes {
    /// Get all built-in palettes
    pub fn get_all() -> Vec<Palette> {
        vec![
            Self::nord(),
            Self::dracula(),
            Self::gruvbox(),
            Self::monokai(),
            Self::solarized(),
            Self::catppuccin(),
            Self::tokyo(),
            Self::oceanic(),
            Self::palenight(),
            Self::onedark(),
            Self::vim(),
            Self::gotham(),
            Self::challenger(),
            Self::molokai(),
            Self::sonokai(),
            Self::serenade(),
            Self::vaporwave(),
        ]
    }

    /// Get a specific palette by name
    pub fn get_palette(name: &str) -> Option<Palette> {
        match name.to_lowercase().as_str() {
            "nord" => Some(Self::nord()),
            "dracula" => Some(Self::dracula()),
            "gruvbox" => Some(Self::gruvbox()),
            "monokai" => Some(Self::monokai()),
            "solarized" => Some(Self::solarized()),
            "catppuccin" => Some(Self::catppuccin()),
            "tokyo" => Some(Self::tokyo()),
            "oceanic" => Some(Self::oceanic()),
            "palenight" => Some(Self::palenight()),
            "onedark" => Some(Self::onedark()),
            "vim" => Some(Self::vim()),
            "gotham" => Some(Self::gotham()),
            "challenger" => Some(Self::challenger()),
            "molokai" => Some(Self::molokai()),
            "sonokai" => Some(Self::sonokai()),
            "serenade" => Some(Self::serenade()),
            "vaporwave" => Some(Self::vaporwave()),
            _ => None,
        }
    }

    /// Get palette names
    pub fn get_names() -> Vec<String> {
        vec![
            "nord".to_string(),
            "dracula".to_string(),
            "gruvbox".to_string(),
            "monokai".to_string(),
            "solarized".to_string(),
            "catppuccin".to_string(),
            "tokyo".to_string(),
            "oceanic".to_string(),
            "palenight".to_string(),
            "onedark".to_string(),
            "vim".to_string(),
            "gotham".to_string(),
            "challenger".to_string(),
            "molokai".to_string(),
            "sonokai".to_string(),
            "serenade".to_string(),
            "vaporwave".to_string(),
        ]
    }

    fn nord() -> Palette {
        Palette {
            name: "nord".to_string(),
            path: PathBuf::from("builtin://nord"),
            colors: vec![
                // Polar Night
                Color {
                    name: "nord0".to_string(),
                    hex: "#2E3440".to_string(),
                },
                Color {
                    name: "nord1".to_string(),
                    hex: "#3B4252".to_string(),
                },
                Color {
                    name: "nord2".to_string(),
                    hex: "#434C5E".to_string(),
                },
                Color {
                    name: "nord3".to_string(),
                    hex: "#4C566A".to_string(),
                },
                // Snow Storm
                Color {
                    name: "nord4".to_string(),
                    hex: "#D8DEE9".to_string(),
                },
                Color {
                    name: "nord5".to_string(),
                    hex: "#E5E9F0".to_string(),
                },
                Color {
                    name: "nord6".to_string(),
                    hex: "#ECEFF4".to_string(),
                },
                // Frost
                Color {
                    name: "nord7".to_string(),
                    hex: "#8FBCBB".to_string(),
                },
                Color {
                    name: "nord8".to_string(),
                    hex: "#88C0D0".to_string(),
                },
                Color {
                    name: "nord9".to_string(),
                    hex: "#81A1C1".to_string(),
                },
                Color {
                    name: "nord10".to_string(),
                    hex: "#5E81AC".to_string(),
                },
                // Aurora
                Color {
                    name: "nord11".to_string(),
                    hex: "#BF616A".to_string(),
                },
                Color {
                    name: "nord12".to_string(),
                    hex: "#D08770".to_string(),
                },
                Color {
                    name: "nord13".to_string(),
                    hex: "#EBCB8B".to_string(),
                },
                Color {
                    name: "nord14".to_string(),
                    hex: "#A3BE8C".to_string(),
                },
                Color {
                    name: "nord15".to_string(),
                    hex: "#B48EAD".to_string(),
                },
            ],
        }
    }

    fn dracula() -> Palette {
        Palette {
            name: "dracula".to_string(),
            path: PathBuf::from("builtin://dracula"),
            colors: vec![
                Color {
                    name: "background".to_string(),
                    hex: "#282A36".to_string(),
                },
                Color {
                    name: "current_line".to_string(),
                    hex: "#44475A".to_string(),
                },
                Color {
                    name: "foreground".to_string(),
                    hex: "#F8F8F2".to_string(),
                },
                Color {
                    name: "comment".to_string(),
                    hex: "#6272A4".to_string(),
                },
                Color {
                    name: "cyan".to_string(),
                    hex: "#8BE9FD".to_string(),
                },
                Color {
                    name: "green".to_string(),
                    hex: "#50FA7B".to_string(),
                },
                Color {
                    name: "orange".to_string(),
                    hex: "#FFB86C".to_string(),
                },
                Color {
                    name: "pink".to_string(),
                    hex: "#FF79C6".to_string(),
                },
                Color {
                    name: "purple".to_string(),
                    hex: "#BD93F9".to_string(),
                },
                Color {
                    name: "red".to_string(),
                    hex: "#FF5555".to_string(),
                },
                Color {
                    name: "yellow".to_string(),
                    hex: "#F1FA8C".to_string(),
                },
            ],
        }
    }

    fn gruvbox() -> Palette {
        Palette {
            name: "gruvbox".to_string(),
            path: PathBuf::from("builtin://gruvbox"),
            colors: vec![
                Color {
                    name: "dark0".to_string(),
                    hex: "#282828".to_string(),
                },
                Color {
                    name: "dark1".to_string(),
                    hex: "#3C3836".to_string(),
                },
                Color {
                    name: "dark2".to_string(),
                    hex: "#504945".to_string(),
                },
                Color {
                    name: "dark3".to_string(),
                    hex: "#665C54".to_string(),
                },
                Color {
                    name: "dark4".to_string(),
                    hex: "#7C6F64".to_string(),
                },
                Color {
                    name: "light0".to_string(),
                    hex: "#FBF1C7".to_string(),
                },
                Color {
                    name: "light1".to_string(),
                    hex: "#EBDBB2".to_string(),
                },
                Color {
                    name: "light2".to_string(),
                    hex: "#D5C4A1".to_string(),
                },
                Color {
                    name: "light3".to_string(),
                    hex: "#BDAE93".to_string(),
                },
                Color {
                    name: "light4".to_string(),
                    hex: "#A89984".to_string(),
                },
                Color {
                    name: "red".to_string(),
                    hex: "#FB4934".to_string(),
                },
                Color {
                    name: "green".to_string(),
                    hex: "#B8BB26".to_string(),
                },
                Color {
                    name: "yellow".to_string(),
                    hex: "#FABD2F".to_string(),
                },
                Color {
                    name: "blue".to_string(),
                    hex: "#83A598".to_string(),
                },
                Color {
                    name: "purple".to_string(),
                    hex: "#D3869B".to_string(),
                },
                Color {
                    name: "aqua".to_string(),
                    hex: "#8EC07C".to_string(),
                },
                Color {
                    name: "orange".to_string(),
                    hex: "#FE8019".to_string(),
                },
            ],
        }
    }

    fn monokai() -> Palette {
        Palette {
            name: "monokai".to_string(),
            path: PathBuf::from("builtin://monokai"),
            colors: vec![
                Color {
                    name: "background".to_string(),
                    hex: "#272822".to_string(),
                },
                Color {
                    name: "foreground".to_string(),
                    hex: "#F8F8F2".to_string(),
                },
                Color {
                    name: "comment".to_string(),
                    hex: "#75715E".to_string(),
                },
                Color {
                    name: "red".to_string(),
                    hex: "#F92672".to_string(),
                },
                Color {
                    name: "orange".to_string(),
                    hex: "#FD971F".to_string(),
                },
                Color {
                    name: "yellow".to_string(),
                    hex: "#E6DB74".to_string(),
                },
                Color {
                    name: "green".to_string(),
                    hex: "#A6E22E".to_string(),
                },
                Color {
                    name: "blue".to_string(),
                    hex: "#66D9EF".to_string(),
                },
                Color {
                    name: "purple".to_string(),
                    hex: "#AE81FF".to_string(),
                },
            ],
        }
    }

    fn solarized() -> Palette {
        Palette {
            name: "solarized".to_string(),
            path: PathBuf::from("builtin://solarized"),
            colors: vec![
                Color {
                    name: "base03".to_string(),
                    hex: "#002B36".to_string(),
                },
                Color {
                    name: "base02".to_string(),
                    hex: "#073642".to_string(),
                },
                Color {
                    name: "base01".to_string(),
                    hex: "#586E75".to_string(),
                },
                Color {
                    name: "base00".to_string(),
                    hex: "#657B83".to_string(),
                },
                Color {
                    name: "base0".to_string(),
                    hex: "#839496".to_string(),
                },
                Color {
                    name: "base1".to_string(),
                    hex: "#93A1A1".to_string(),
                },
                Color {
                    name: "base2".to_string(),
                    hex: "#EEE8D5".to_string(),
                },
                Color {
                    name: "base3".to_string(),
                    hex: "#FDF6E3".to_string(),
                },
                Color {
                    name: "yellow".to_string(),
                    hex: "#B58900".to_string(),
                },
                Color {
                    name: "orange".to_string(),
                    hex: "#CB4B16".to_string(),
                },
                Color {
                    name: "red".to_string(),
                    hex: "#DC322F".to_string(),
                },
                Color {
                    name: "magenta".to_string(),
                    hex: "#D33682".to_string(),
                },
                Color {
                    name: "violet".to_string(),
                    hex: "#6C71C4".to_string(),
                },
                Color {
                    name: "blue".to_string(),
                    hex: "#268BD2".to_string(),
                },
                Color {
                    name: "cyan".to_string(),
                    hex: "#2AA198".to_string(),
                },
                Color {
                    name: "green".to_string(),
                    hex: "#859900".to_string(),
                },
            ],
        }
    }

    fn catppuccin() -> Palette {
        Palette {
            name: "catppuccin".to_string(),
            path: PathBuf::from("builtin://catppuccin"),
            colors: vec![
                Color {
                    name: "rosewater".to_string(),
                    hex: "#F5E0DC".to_string(),
                },
                Color {
                    name: "flamingo".to_string(),
                    hex: "#F2CDCD".to_string(),
                },
                Color {
                    name: "pink".to_string(),
                    hex: "#F5C2E7".to_string(),
                },
                Color {
                    name: "mauve".to_string(),
                    hex: "#CBA6F7".to_string(),
                },
                Color {
                    name: "red".to_string(),
                    hex: "#F38BA8".to_string(),
                },
                Color {
                    name: "maroon".to_string(),
                    hex: "#EBA0AC".to_string(),
                },
                Color {
                    name: "peach".to_string(),
                    hex: "#FAB387".to_string(),
                },
                Color {
                    name: "yellow".to_string(),
                    hex: "#F9E2AF".to_string(),
                },
                Color {
                    name: "green".to_string(),
                    hex: "#A6E3A1".to_string(),
                },
                Color {
                    name: "teal".to_string(),
                    hex: "#94E2D5".to_string(),
                },
                Color {
                    name: "sky".to_string(),
                    hex: "#89DCEB".to_string(),
                },
                Color {
                    name: "sapphire".to_string(),
                    hex: "#74C7EC".to_string(),
                },
                Color {
                    name: "blue".to_string(),
                    hex: "#89B4FA".to_string(),
                },
                Color {
                    name: "lavender".to_string(),
                    hex: "#B4BEFE".to_string(),
                },
                Color {
                    name: "text".to_string(),
                    hex: "#CDD6F4".to_string(),
                },
                Color {
                    name: "base".to_string(),
                    hex: "#1E1E2E".to_string(),
                },
            ],
        }
    }

    fn tokyo() -> Palette {
        Palette {
            name: "tokyo".to_string(),
            path: PathBuf::from("builtin://tokyo"),
            colors: vec![
                Color {
                    name: "bg".to_string(),
                    hex: "#1A1B26".to_string(),
                },
                Color {
                    name: "bg_dark".to_string(),
                    hex: "#16161E".to_string(),
                },
                Color {
                    name: "bg_highlight".to_string(),
                    hex: "#292E42".to_string(),
                },
                Color {
                    name: "terminal_black".to_string(),
                    hex: "#414868".to_string(),
                },
                Color {
                    name: "fg".to_string(),
                    hex: "#C0CAF5".to_string(),
                },
                Color {
                    name: "fg_dark".to_string(),
                    hex: "#A9B1D6".to_string(),
                },
                Color {
                    name: "fg_gutter".to_string(),
                    hex: "#3B4261".to_string(),
                },
                Color {
                    name: "dark3".to_string(),
                    hex: "#545C7E".to_string(),
                },
                Color {
                    name: "comment".to_string(),
                    hex: "#565F89".to_string(),
                },
                Color {
                    name: "dark5".to_string(),
                    hex: "#737AA2".to_string(),
                },
                Color {
                    name: "blue0".to_string(),
                    hex: "#3D59A1".to_string(),
                },
                Color {
                    name: "blue".to_string(),
                    hex: "#7AA2F7".to_string(),
                },
                Color {
                    name: "cyan".to_string(),
                    hex: "#7DCFFF".to_string(),
                },
                Color {
                    name: "blue1".to_string(),
                    hex: "#2AC3DE".to_string(),
                },
                Color {
                    name: "blue2".to_string(),
                    hex: "#0DB9D7".to_string(),
                },
                Color {
                    name: "blue5".to_string(),
                    hex: "#89DDFF".to_string(),
                },
                Color {
                    name: "blue6".to_string(),
                    hex: "#B4F9F8".to_string(),
                },
                Color {
                    name: "blue7".to_string(),
                    hex: "#394B70".to_string(),
                },
                Color {
                    name: "magenta".to_string(),
                    hex: "#BB9AF7".to_string(),
                },
                Color {
                    name: "magenta2".to_string(),
                    hex: "#FF007C".to_string(),
                },
                Color {
                    name: "purple".to_string(),
                    hex: "#9D7CD8".to_string(),
                },
                Color {
                    name: "orange".to_string(),
                    hex: "#FF9E64".to_string(),
                },
                Color {
                    name: "yellow".to_string(),
                    hex: "#E0AF68".to_string(),
                },
                Color {
                    name: "green".to_string(),
                    hex: "#9ECE6A".to_string(),
                },
                Color {
                    name: "green1".to_string(),
                    hex: "#73DACA".to_string(),
                },
                Color {
                    name: "green2".to_string(),
                    hex: "#41A6B5".to_string(),
                },
                Color {
                    name: "teal".to_string(),
                    hex: "#1ABC9C".to_string(),
                },
                Color {
                    name: "red".to_string(),
                    hex: "#F7768E".to_string(),
                },
                Color {
                    name: "red1".to_string(),
                    hex: "#DB4B4B".to_string(),
                },
            ],
        }
    }

    // Simplified palettes for the remaining themes
    fn oceanic() -> Palette {
        Palette {
            name: "oceanic".to_string(),
            path: PathBuf::from("builtin://oceanic"),
            colors: vec![
                Color {
                    name: "base00".to_string(),
                    hex: "#2B303B".to_string(),
                },
                Color {
                    name: "base01".to_string(),
                    hex: "#343D46".to_string(),
                },
                Color {
                    name: "base02".to_string(),
                    hex: "#4F5B66".to_string(),
                },
                Color {
                    name: "base03".to_string(),
                    hex: "#65737E".to_string(),
                },
                Color {
                    name: "base04".to_string(),
                    hex: "#A7ADBA".to_string(),
                },
                Color {
                    name: "base05".to_string(),
                    hex: "#C0C5CE".to_string(),
                },
                Color {
                    name: "base06".to_string(),
                    hex: "#DFE1E8".to_string(),
                },
                Color {
                    name: "base07".to_string(),
                    hex: "#EFF1F5".to_string(),
                },
                Color {
                    name: "base08".to_string(),
                    hex: "#BF616A".to_string(),
                },
                Color {
                    name: "base09".to_string(),
                    hex: "#D08770".to_string(),
                },
                Color {
                    name: "base0A".to_string(),
                    hex: "#EBCB8B".to_string(),
                },
                Color {
                    name: "base0B".to_string(),
                    hex: "#A3BE8C".to_string(),
                },
                Color {
                    name: "base0C".to_string(),
                    hex: "#96B5B4".to_string(),
                },
                Color {
                    name: "base0D".to_string(),
                    hex: "#8FA1B3".to_string(),
                },
                Color {
                    name: "base0E".to_string(),
                    hex: "#B48EAD".to_string(),
                },
                Color {
                    name: "base0F".to_string(),
                    hex: "#AB7967".to_string(),
                },
            ],
        }
    }

    fn palenight() -> Palette {
        Palette {
            name: "palenight".to_string(),
            path: PathBuf::from("builtin://palenight"),
            colors: vec![
                Color {
                    name: "background".to_string(),
                    hex: "#292D3E".to_string(),
                },
                Color {
                    name: "foreground".to_string(),
                    hex: "#BFC7D5".to_string(),
                },
                Color {
                    name: "cursor".to_string(),
                    hex: "#FFCC00".to_string(),
                },
                Color {
                    name: "red".to_string(),
                    hex: "#F07178".to_string(),
                },
                Color {
                    name: "green".to_string(),
                    hex: "#C3E88D".to_string(),
                },
                Color {
                    name: "yellow".to_string(),
                    hex: "#FFCB6B".to_string(),
                },
                Color {
                    name: "blue".to_string(),
                    hex: "#82AAFF".to_string(),
                },
                Color {
                    name: "magenta".to_string(),
                    hex: "#C792EA".to_string(),
                },
                Color {
                    name: "cyan".to_string(),
                    hex: "#89DDFF".to_string(),
                },
                Color {
                    name: "white".to_string(),
                    hex: "#EEFFFF".to_string(),
                },
            ],
        }
    }

    fn onedark() -> Palette {
        Palette {
            name: "onedark".to_string(),
            path: PathBuf::from("builtin://onedark"),
            colors: vec![
                Color {
                    name: "black".to_string(),
                    hex: "#282C34".to_string(),
                },
                Color {
                    name: "red".to_string(),
                    hex: "#E06C75".to_string(),
                },
                Color {
                    name: "green".to_string(),
                    hex: "#98C379".to_string(),
                },
                Color {
                    name: "yellow".to_string(),
                    hex: "#E5C07B".to_string(),
                },
                Color {
                    name: "blue".to_string(),
                    hex: "#61AFEF".to_string(),
                },
                Color {
                    name: "magenta".to_string(),
                    hex: "#C678DD".to_string(),
                },
                Color {
                    name: "cyan".to_string(),
                    hex: "#56B6C2".to_string(),
                },
                Color {
                    name: "white".to_string(),
                    hex: "#ABB2BF".to_string(),
                },
            ],
        }
    }

    fn vim() -> Palette {
        Palette {
            name: "vim".to_string(),
            path: PathBuf::from("builtin://vim"),
            colors: vec![
                Color {
                    name: "black".to_string(),
                    hex: "#000000".to_string(),
                },
                Color {
                    name: "dark_red".to_string(),
                    hex: "#800000".to_string(),
                },
                Color {
                    name: "dark_green".to_string(),
                    hex: "#008000".to_string(),
                },
                Color {
                    name: "dark_yellow".to_string(),
                    hex: "#808000".to_string(),
                },
                Color {
                    name: "dark_blue".to_string(),
                    hex: "#000080".to_string(),
                },
                Color {
                    name: "dark_magenta".to_string(),
                    hex: "#800080".to_string(),
                },
                Color {
                    name: "dark_cyan".to_string(),
                    hex: "#008080".to_string(),
                },
                Color {
                    name: "gray".to_string(),
                    hex: "#C0C0C0".to_string(),
                },
                Color {
                    name: "dark_gray".to_string(),
                    hex: "#808080".to_string(),
                },
                Color {
                    name: "red".to_string(),
                    hex: "#FF0000".to_string(),
                },
                Color {
                    name: "green".to_string(),
                    hex: "#00FF00".to_string(),
                },
                Color {
                    name: "yellow".to_string(),
                    hex: "#FFFF00".to_string(),
                },
                Color {
                    name: "blue".to_string(),
                    hex: "#0000FF".to_string(),
                },
                Color {
                    name: "magenta".to_string(),
                    hex: "#FF00FF".to_string(),
                },
                Color {
                    name: "cyan".to_string(),
                    hex: "#00FFFF".to_string(),
                },
                Color {
                    name: "white".to_string(),
                    hex: "#FFFFFF".to_string(),
                },
            ],
        }
    }

    fn gotham() -> Palette {
        Palette {
            name: "gotham".to_string(),
            path: PathBuf::from("builtin://gotham"),
            colors: vec![
                Color {
                    name: "base00".to_string(),
                    hex: "#0C1014".to_string(),
                },
                Color {
                    name: "base01".to_string(),
                    hex: "#11151C".to_string(),
                },
                Color {
                    name: "base02".to_string(),
                    hex: "#091F2E".to_string(),
                },
                Color {
                    name: "base03".to_string(),
                    hex: "#0A3749".to_string(),
                },
                Color {
                    name: "base04".to_string(),
                    hex: "#245361".to_string(),
                },
                Color {
                    name: "base05".to_string(),
                    hex: "#599CAB".to_string(),
                },
                Color {
                    name: "base06".to_string(),
                    hex: "#99D1CE".to_string(),
                },
                Color {
                    name: "base07".to_string(),
                    hex: "#D3EBE9".to_string(),
                },
                Color {
                    name: "base08".to_string(),
                    hex: "#C33027".to_string(),
                },
                Color {
                    name: "base09".to_string(),
                    hex: "#D26939".to_string(),
                },
                Color {
                    name: "base0A".to_string(),
                    hex: "#EDB54B".to_string(),
                },
                Color {
                    name: "base0B".to_string(),
                    hex: "#2AA889".to_string(),
                },
                Color {
                    name: "base0C".to_string(),
                    hex: "#33859D".to_string(),
                },
                Color {
                    name: "base0D".to_string(),
                    hex: "#195465".to_string(),
                },
                Color {
                    name: "base0E".to_string(),
                    hex: "#888CA6".to_string(),
                },
                Color {
                    name: "base0F".to_string(),
                    hex: "#4E5166".to_string(),
                },
            ],
        }
    }

    fn challenger() -> Palette {
        Palette {
            name: "challenger".to_string(),
            path: PathBuf::from("builtin://challenger"),
            colors: vec![
                Color {
                    name: "background".to_string(),
                    hex: "#1B1D29".to_string(),
                },
                Color {
                    name: "foreground".to_string(),
                    hex: "#CBE3E7".to_string(),
                },
                Color {
                    name: "black".to_string(),
                    hex: "#100E23".to_string(),
                },
                Color {
                    name: "red".to_string(),
                    hex: "#FF8080".to_string(),
                },
                Color {
                    name: "green".to_string(),
                    hex: "#95FFA4".to_string(),
                },
                Color {
                    name: "yellow".to_string(),
                    hex: "#FFE9AA".to_string(),
                },
                Color {
                    name: "blue".to_string(),
                    hex: "#91DDFF".to_string(),
                },
                Color {
                    name: "magenta".to_string(),
                    hex: "#C991E1".to_string(),
                },
                Color {
                    name: "cyan".to_string(),
                    hex: "#AAFFE4".to_string(),
                },
                Color {
                    name: "white".to_string(),
                    hex: "#CBE3E7".to_string(),
                },
            ],
        }
    }

    fn molokai() -> Palette {
        Palette {
            name: "molokai".to_string(),
            path: PathBuf::from("builtin://molokai"),
            colors: vec![
                Color {
                    name: "black".to_string(),
                    hex: "#1B1D1E".to_string(),
                },
                Color {
                    name: "red".to_string(),
                    hex: "#F92672".to_string(),
                },
                Color {
                    name: "green".to_string(),
                    hex: "#A6E22E".to_string(),
                },
                Color {
                    name: "yellow".to_string(),
                    hex: "#E6DB74".to_string(),
                },
                Color {
                    name: "blue".to_string(),
                    hex: "#66D9EF".to_string(),
                },
                Color {
                    name: "magenta".to_string(),
                    hex: "#AE81FF".to_string(),
                },
                Color {
                    name: "cyan".to_string(),
                    hex: "#A1EFE4".to_string(),
                },
                Color {
                    name: "white".to_string(),
                    hex: "#F8F8F2".to_string(),
                },
            ],
        }
    }

    fn sonokai() -> Palette {
        Palette {
            name: "sonokai".to_string(),
            path: PathBuf::from("builtin://sonokai"),
            colors: vec![
                Color {
                    name: "bg0".to_string(),
                    hex: "#2C2E34".to_string(),
                },
                Color {
                    name: "bg1".to_string(),
                    hex: "#33353F".to_string(),
                },
                Color {
                    name: "bg2".to_string(),
                    hex: "#363944".to_string(),
                },
                Color {
                    name: "bg3".to_string(),
                    hex: "#3B3E48".to_string(),
                },
                Color {
                    name: "bg4".to_string(),
                    hex: "#414550".to_string(),
                },
                Color {
                    name: "fg".to_string(),
                    hex: "#E2E2E3".to_string(),
                },
                Color {
                    name: "red".to_string(),
                    hex: "#FC5D7C".to_string(),
                },
                Color {
                    name: "orange".to_string(),
                    hex: "#F39660".to_string(),
                },
                Color {
                    name: "yellow".to_string(),
                    hex: "#E7C664".to_string(),
                },
                Color {
                    name: "green".to_string(),
                    hex: "#9ED072".to_string(),
                },
                Color {
                    name: "blue".to_string(),
                    hex: "#76CCE0".to_string(),
                },
                Color {
                    name: "purple".to_string(),
                    hex: "#B39DF3".to_string(),
                },
            ],
        }
    }

    fn serenade() -> Palette {
        Palette {
            name: "serenade".to_string(),
            path: PathBuf::from("builtin://serenade"),
            colors: vec![
                Color {
                    name: "background".to_string(),
                    hex: "#303340".to_string(),
                },
                Color {
                    name: "foreground".to_string(),
                    hex: "#D4D4D6".to_string(),
                },
                Color {
                    name: "black".to_string(),
                    hex: "#3A3D4A".to_string(),
                },
                Color {
                    name: "red".to_string(),
                    hex: "#D76E6E".to_string(),
                },
                Color {
                    name: "green".to_string(),
                    hex: "#B1D196".to_string(),
                },
                Color {
                    name: "yellow".to_string(),
                    hex: "#F9E79F".to_string(),
                },
                Color {
                    name: "blue".to_string(),
                    hex: "#8BB8DF".to_string(),
                },
                Color {
                    name: "magenta".to_string(),
                    hex: "#BB97EE".to_string(),
                },
                Color {
                    name: "cyan".to_string(),
                    hex: "#9FE7DD".to_string(),
                },
                Color {
                    name: "white".to_string(),
                    hex: "#D4D4D6".to_string(),
                },
            ],
        }
    }

    fn vaporwave() -> Palette {
        Palette {
            name: "vaporwave".to_string(),
            path: PathBuf::from("builtin://vaporwave"),
            colors: vec![
                Color {
                    name: "background".to_string(),
                    hex: "#0F0F23".to_string(),
                },
                Color {
                    name: "foreground".to_string(),
                    hex: "#FAFAFA".to_string(),
                },
                Color {
                    name: "pink".to_string(),
                    hex: "#F92672".to_string(),
                },
                Color {
                    name: "purple".to_string(),
                    hex: "#AE81FF".to_string(),
                },
                Color {
                    name: "cyan".to_string(),
                    hex: "#66D9EF".to_string(),
                },
                Color {
                    name: "green".to_string(),
                    hex: "#A6E22E".to_string(),
                },
                Color {
                    name: "yellow".to_string(),
                    hex: "#E6DB74".to_string(),
                },
                Color {
                    name: "orange".to_string(),
                    hex: "#FD971F".to_string(),
                },
                Color {
                    name: "red".to_string(),
                    hex: "#F92672".to_string(),
                },
                Color {
                    name: "blue".to_string(),
                    hex: "#66D9EF".to_string(),
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_palettes() {
        let palettes = BuiltinPalettes::get_all();
        assert_eq!(palettes.len(), 17);
        assert!(palettes.iter().any(|p| p.name == "nord"));
        assert!(palettes.iter().any(|p| p.name == "dracula"));
    }

    #[test]
    fn test_get_palette_by_name() {
        let nord = BuiltinPalettes::get_palette("nord").unwrap();
        assert_eq!(nord.name, "nord");
        assert_eq!(nord.colors.len(), 16);
        assert_eq!(nord.colors[0].hex, "#2E3440");

        let dracula = BuiltinPalettes::get_palette("dracula").unwrap();
        assert_eq!(dracula.name, "dracula");
        assert!(!dracula.colors.is_empty());
    }

    #[test]
    fn test_get_palette_names() {
        let names = BuiltinPalettes::get_names();
        assert_eq!(names.len(), 17);
        assert!(names.contains(&"nord".to_string()));
        assert!(names.contains(&"dracula".to_string()));
    }

    #[test]
    fn test_case_insensitive_lookup() {
        assert!(BuiltinPalettes::get_palette("NORD").is_some());
        assert!(BuiltinPalettes::get_palette("Nord").is_some());
        assert!(BuiltinPalettes::get_palette("nord").is_some());
    }

    #[test]
    fn test_unknown_palette() {
        assert!(BuiltinPalettes::get_palette("unknown").is_none());
    }
}

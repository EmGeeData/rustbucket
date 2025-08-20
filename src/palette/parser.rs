use crate::{Color, RustBucketError};

pub fn parse_hex_color(hex: &str, name: String) -> crate::Result<Color> {
    let hex = hex.trim();

    if hex.is_empty() {
        return Err(RustBucketError::InvalidColor(
            "Empty color string".to_string(),
        ));
    }

    let hex_clean = hex.strip_prefix('#').unwrap_or(hex);

    if hex_clean.len() != 6 {
        return Err(RustBucketError::InvalidColor(format!(
            "Invalid hex color length: {} (expected 6 characters)",
            hex_clean
        )));
    }

    if !hex_clean.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(RustBucketError::InvalidColor(format!(
            "Invalid hex color format: {}",
            hex
        )));
    }

    let hex_with_hash = if hex.starts_with('#') {
        hex.to_string()
    } else {
        format!("#{}", hex)
    };

    Ok(Color {
        name,
        hex: hex_with_hash,
    })
}

#[allow(dead_code)]
pub fn hex_to_rgb(hex: &str) -> crate::Result<(u8, u8, u8)> {
    let hex_clean = hex.strip_prefix('#').unwrap_or(hex);

    if hex_clean.len() != 6 {
        return Err(RustBucketError::InvalidColor(format!(
            "Invalid hex color length for RGB conversion: {}",
            hex
        )));
    }

    let r = u8::from_str_radix(&hex_clean[0..2], 16).map_err(|_| {
        RustBucketError::InvalidColor(format!("Invalid red component: {}", &hex_clean[0..2]))
    })?;
    let g = u8::from_str_radix(&hex_clean[2..4], 16).map_err(|_| {
        RustBucketError::InvalidColor(format!("Invalid green component: {}", &hex_clean[2..4]))
    })?;
    let b = u8::from_str_radix(&hex_clean[4..6], 16).map_err(|_| {
        RustBucketError::InvalidColor(format!("Invalid blue component: {}", &hex_clean[4..6]))
    })?;

    Ok((r, g, b))
}

#[allow(dead_code)]
pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color_with_hash() {
        let color = parse_hex_color("#BF616A", "Red".to_string()).unwrap();
        assert_eq!(color.name, "Red");
        assert_eq!(color.hex, "#BF616A");
    }

    #[test]
    fn test_parse_hex_color_without_hash() {
        let color = parse_hex_color("BF616A", "Red".to_string()).unwrap();
        assert_eq!(color.name, "Red");
        assert_eq!(color.hex, "#BF616A");
    }

    #[test]
    fn test_parse_hex_color_lowercase() {
        let color = parse_hex_color("bf616a", "Red".to_string()).unwrap();
        assert_eq!(color.name, "Red");
        assert_eq!(color.hex, "#bf616a");
    }

    #[test]
    fn test_parse_hex_color_invalid_length() {
        assert!(parse_hex_color("BF61", "Red".to_string()).is_err());
        assert!(parse_hex_color("BF616A1", "Red".to_string()).is_err());
    }

    #[test]
    fn test_parse_hex_color_invalid_chars() {
        assert!(parse_hex_color("GGGGGG", "Red".to_string()).is_err());
        assert!(parse_hex_color("BF616Z", "Red".to_string()).is_err());
    }

    #[test]
    fn test_parse_hex_color_empty() {
        assert!(parse_hex_color("", "Red".to_string()).is_err());
        assert!(parse_hex_color("   ", "Red".to_string()).is_err());
    }

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#BF616A").unwrap(), (191, 97, 106));
        assert_eq!(hex_to_rgb("BF616A").unwrap(), (191, 97, 106));
        assert_eq!(hex_to_rgb("#000000").unwrap(), (0, 0, 0));
        assert_eq!(hex_to_rgb("#FFFFFF").unwrap(), (255, 255, 255));
    }

    #[test]
    fn test_rgb_to_hex() {
        assert_eq!(rgb_to_hex(191, 97, 106), "#BF616A");
        assert_eq!(rgb_to_hex(0, 0, 0), "#000000");
        assert_eq!(rgb_to_hex(255, 255, 255), "#FFFFFF");
    }
}

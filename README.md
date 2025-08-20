# RustBucket

A blazing-fast, memory-efficient CLI tool for converting images to themed color palettes.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Rust](https://img.shields.io/badge/Made%20with-Rust-orange.svg)](https://www.rust-lang.org/)

## Why RustBucket?

RustBucket is a high-performance CLI tool for converting images to themed color palettes, built with Rust for maximum efficiency:

- **Performance**: Blazing-fast image processing with optimized algorithms
- **Memory Safety**: Zero-cost abstractions with guaranteed memory safety
- **Efficiency**: Minimal memory footprint and resource usage
- **Features**: Comprehensive palette management and image processing effects
- **Single Binary**: No runtime dependencies or interpreter required

## Installation

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (1.70.0 or later)

### Build from Source

```bash
git clone https://github.com/emgeedata/rustbucket
cd rustbucket
cargo build --release
```

The optimized binary will be available at `./target/release/rtbt`.

### Shell Completions

Generate shell completion files for enhanced command-line experience:

```bash
cargo run --bin generate-completions
```

This creates completion files in shell-specific directories under `completions/`:

- **Bash**: `completions/bash/rtbt.bash`
- **Zsh**: `completions/zsh/_rtbt`
- **Fish**: `completions/fish/rtbt.fish`
- **PowerShell**: `completions/powershell/_rtbt.ps1`

The completions provide tab-completion support for:

- All command-line options and flags
- Built-in palette names
- File path completion

See `completions/README.md` for installation instructions.

## Quick Start

```bash
# Convert image with default Nord palette
./target/release/rtbt -i photo.jpg -o result.png

# Use Dracula palette with blur effect
./target/release/rtbt -i image.png -o dark.png -p dracula --blur

# Performance benchmark mode
./target/release/rtbt -i test.jpg -o output.jpg --benchmark
```

## Usage

### Basic Commands

```bash
# Simple conversion
rtbt -i input.png -o output.png

# Choose different palette
rtbt -i photo.jpg -o themed.png --palette gruvbox

# Apply Gaussian blur
rtbt -i image.png -o blurred.png --blur
```

### Performance Options

```bash
# Fast processing (disable pixel averaging)
rtbt -i large.jpg -o fast.png --no-avg

# Custom pixel area for quality/speed balance
rtbt -i image.png -o balanced.png --pixels-area 2,2

# Quiet mode
rtbt -i input.png -o output.png --quiet
```

### Benchmarking

```bash
rtbt -i test.jpg -o result.jpg --benchmark
```

Example benchmark output:

```text
Benchmark Results:
  Total processing: 0.08s
    - Load time:    3ms
    - Conversion:   67ms
    - Effects:      8ms
    - Save time:    2ms
  Pixels processed: 307200 (3840.0K pixels/s)

Performance Grade: Excellent

Optimization Suggestions:
  - Configuration looks well optimized for performance
  - Current settings provide excellent speed-to-quality ratio
```

## Available Palettes

RustBucket includes **17 built-in color themes** compiled directly into the binary:

```bash
# List all available palettes
rtbt --list-palettes

# Create a skeleton palette file
rtbt --create-palette my_theme.toml

# Export a built-in palette to TOML for customization
rtbt --export-palette nord nord_custom.toml
```

| Theme            | Description                                       |
| ---------------- | ------------------------------------------------- |
| `nord` (default) | Arctic, north-bluish clean palette with 16 colors |
| `dracula`        | Dark theme with vibrant accent colors (11 colors) |
| `gruvbox`        | Retro groove warm color scheme (17 colors)        |
| `solarized`      | Precision engineered color palette (16 colors)    |
| `catppuccin`     | Soothing pastel theme (16 colors)                 |
| `monokai`        | Popular dark coding theme (9 colors)              |
| `tokyo`          | Clean Tokyo Night inspired theme (29 colors)      |
| `oceanic`        | Deep ocean blue tones (16 colors)                 |
| `palenight`      | Elegant dark purple theme (10 colors)             |
| `onedark`        | Atom's One Dark theme (8 colors)                  |
| `vim`            | Classic Vim editor colors (16 colors)             |
| `gotham`         | Dark, Batman-inspired theme (16 colors)           |
| `challenger`     | High-contrast dark theme (10 colors)              |
| `molokai`        | Molokai terminal theme (8 colors)                 |
| `sonokai`        | High-contrast color scheme (12 colors)            |
| `serenade`       | Calm, balanced color palette (10 colors)          |
| `vaporwave`      | Retro synthwave aesthetic (10 colors)             |

### Custom TOML Palettes

Create your own palettes using TOML format. rtbt searches for palettes in multiple locations (in order):

1. **`RTBT_PALETTE_DIR`** environment variable path (if set)
2. **`~/.config/rtbt/palettes/`** (user config directory)

The first palette found with a given name takes precedence.

```toml
name = "example"
description = "An example custom palette for image tinting"
author = "Your Name"

[[colors]]
name = "dark_blue"
hex = "#2E3440"
description = "Deep blue-grey tone"

[[colors]]
name = "light_blue"
hex = "#88C0D0"
description = "Soft cyan-blue"

[[colors]]
name = "green"
hex = "#A3BE8C"
description = "Muted green"

[[colors]]
name = "yellow"
hex = "#EBCB8B"
description = "Warm yellow"

[[colors]]
name = "orange"
hex = "#D08770"
description = "Soft orange"

[[colors]]
name = "red"
hex = "#BF616A"
description = "Muted red"

[[colors]]
name = "purple"
hex = "#B48EAD"
description = "Soft purple"

[[colors]]
name = "white"
hex = "#ECEFF4"
description = "Light neutral"
```

### Creating Custom Palettes

rtbt makes it easy to create custom palettes for image tinting:

```bash
# Option 1: Create a skeleton palette file with 8 example colors
rtbt --create-palette my_theme.toml

# Option 2: Export an existing built-in palette as starting point
rtbt --export-palette nord my_nord_variant.toml

# Move to appropriate directory and customize
mkdir -p palettes
mv my_theme.toml palettes/
# Edit palettes/my_theme.toml with your desired colors

# Use your custom palette
rtbt -i input.png -o output.png -p my_theme
```

The generated skeleton includes realistic color names for image tinting:

- `dark_blue`, `light_blue` - Various blue tones
- `green`, `yellow`, `orange`, `red` - Primary colors
- `purple` - Accent color
- `white` - Light neutral tone

Each color includes hex values and descriptions to help you customize the palette effectively.

### Usage Examples

```bash
# Use system-wide palettes
mkdir -p ~/.config/rtbt/palettes
rtbt --create-palette ~/.config/rtbt/palettes/mytheme.toml

# Use project-specific palettes
mkdir palettes
rtbt --create-palette palettes/project_theme.toml

# Use custom directory via environment variable
export RTBT_PALETTE_DIR="/path/to/my/palettes"
rtbt --create-palette /path/to/my/palettes/custom_palette.toml
rtbt -i input.png -o output.png -p custom_palette
```

TOML palettes can override built-in themes by using the same name.

## Command Line Options

| Option                              | Short | Description                                          |
| ----------------------------------- | ----- | ---------------------------------------------------- |
| `--img <PATH>`                      | `-i`  | Input image file (required)                          |
| `--out <PATH>`                      | `-o`  | Output image file (default: nord.png)                |
| `--palette <NAME>`                  | `-p`  | Palette name (default: nord)                         |
| `--colors <LIST>`                   | `-c`  | Specific colors (comma-separated)                    |
| `--blur`                            | `-b`  | Apply Gaussian blur effect                           |
| `--quiet`                           | `-q`  | Suppress output messages                             |
| `--no-avg`                          |       | Disable pixel averaging (faster)                     |
| `--pixels-area <W,H>`               |       | Custom pixel area size                               |
| `--benchmark`                       |       | Run performance analysis                             |
| `--list-palettes`                   |       | List all available built-in and user palettes        |
| `--create-palette <PATH>`           |       | Create a skeleton palette file at the specified path |
| `--export-palette <PALETTE> <PATH>` |       | Export a built-in palette to TOML format             |
| `--help`                            | `-h`  | Show help information                                |
| `--version`                         | `-V`  | Show version                                         |

## Development

### Project Structure

```text
src/
- main.rs          # Application entry point
- lib.rs           # Public API exports
- cli.rs           # Command-line parsing (clap)
- error.rs         # Error handling (thiserror)
- palette/         # Palette management
  - builtin.rs     # 17 built-in palettes as constants
  - loader.rs      # Legacy file system operations
  - parser.rs      # Color parsing & validation
  - toml_loader.rs # TOML palette loading/saving
  - mod.rs         # Public exports
- image/           # Image processing pipeline
  - converter.rs   # Color space conversion
  - processor.rs   # Main processing logic
  - effects.rs     # Visual effects (blur)
  - benchmark.rs   # Performance analysis
  - mod.rs         # Public exports

completions/       # Shell completion files
packaging/         # Multi-platform package definitions
scripts/           # Build and utility scripts
tests/             # Integration tests
```

### Running Tests

```bash
# All tests (55 total: 50 unit + 5 integration)
cargo test

# Unit tests only (50 tests)
cargo test --lib

# Integration tests (5 tests)
cargo test --test integration_test

# With output
cargo test -- --nocapture
```

### Adding New Palettes

1. Create TOML palette file: `rtbt --create-palette my_theme.toml`
2. Edit the generated file with your custom colors
3. Place in one of the search locations (`~/.config/rtbt/palettes/` or custom directory)
4. Test: `cargo test`

The generated skeleton includes all required TOML structure with example colors that you can customize.

### Packaging

See the `packaging/` directory for platform-specific package definitions:

- **Arch Linux**: `packaging/arch/PKGBUILD` for AUR submission
- **macOS**: `packaging/homebrew/rtbt.rb` for Homebrew formula
- **Testing**: `packaging/scripts/` contains validation scripts

Run the packaging test suite:

```bash
cd packaging/scripts
./test-all.sh
```

## License

Licensed under the MIT License. See [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspiration [ImageGoNordCLI](https://github.com/schroedinger-Hat/ImageGoNord-cli) project team
- Color theme communities:
  - [Nord Theme](https://www.nordtheme.com/) community
  - [Dracula Theme](https://draculatheme.com/) contributors
  - [Gruvbox](https://github.com/morhetz/gruvbox) theme creators
  - [Solarized](https://ethanschoonover.com/solarized/) by Ethan Schoonover
  - [Catppuccin](https://catppuccin.com/) community
  - [Tokyo Night](https://github.com/enkia/tokyo-night-vscode-theme) theme creators
  - [One Dark](https://github.com/atom/atom/tree/master/packages/one-dark-syntax) theme contributors
- Key Rust crates:
  - [clap](https://crates.io/crates/clap) for command-line parsing
  - [image](https://crates.io/crates/image) for image processing
  - [serde](https://crates.io/crates/serde) for serialization
  - [toml](https://crates.io/crates/toml) for configuration parsing
- Rust community for excellent tooling and ecosystem

# RustBucket Packaging

This directory contains package definitions for distributing RustBucket (`rtbt`) across multiple platforms.

## Available Packages

| Platform | Package Manager | Status | Installation |
|----------|----------------|---------|-------------|
| **Arch Linux** | AUR | 🚧 Planned | `yay -S rustbucket` |
| **macOS** | Homebrew | 🚧 Planned | `brew install rtbt` |

## Directory Structure

```tree
packaging/
├── README.md              # This file - packaging overview
├── PACKAGING_PLAN.md      # Detailed implementation plan
├── arch/                  # Arch Linux (AUR) packaging
├── homebrew/              # macOS Homebrew formula
└── scripts/               # Testing and build scripts
```

## Quick Start

### For Package Maintainers

1. **Choose your platform** from the directories above
2. **Read the platform-specific README** for detailed instructions
3. **Test locally** using the provided scripts
4. **Follow submission guidelines** for your platform

### For Users

See the main [README.md](../README.md) for installation instructions or build from source:

```bash
git clone https://github.com/emgeedata/rustbucket
cd rustbucket
cargo build --release
```

## Project Information

- **Name**: rustbucket
- **Binary**: rtbt
- **Version**: 1.0.0
- **License**: MIT
- **Repository**: <https://github.com/emgeedata/rustbucket>
- **Description**: Blazing-fast, memory-efficient CLI tool for converting images to themed color palettes

## Features

- **Zero Dependencies**: Statically linked Rust binary
- **Shell Completions**: Bash, Zsh, Fish, PowerShell
- **Cross-Platform**: Linux, macOS, Windows
- **High Performance**: Optimized Rust implementation
- **17 Built-in Palettes**: Nord, Dracula, Gruvbox, and more

## Package Testing

Each platform includes local testing scripts:

```bash
# Test all packages
./scripts/test-all.sh

# Test specific platform
./scripts/test-arch.sh
./scripts/test-homebrew.sh
```

## Contributing

1. Follow platform-specific packaging guidelines
2. Test packages locally before submission
3. Update version numbers and checksums for new releases
4. Submit to appropriate repositories following their procedures

## Support

- **Issues**: [GitHub Issues](https://github.com/emgeedata/rustbucket/issues)
- **Discussions**: [GitHub Discussions](https://github.com/emgeedata/rustbucket/discussions)
- **Documentation**: See individual platform READMEs

## Status Legend

- ✅ **Available**: Package is published and available
- 🚧 **Planned**: Package definition created, pending submission
- 📝 **In Development**: Package being created
- ❌ **Unavailable**: Platform not supported or discontinued

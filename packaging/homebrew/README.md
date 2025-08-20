# macOS Package (Homebrew)

This directory contains the Homebrew formula for distributing RustBucket on macOS.

## Package Information

- **Formula Name**: `rtbt`
- **Tap**: `emgeedata/tap` (initially) or `homebrew/core` (eventual goal)
- **Build Type**: Source build using cargo
- **Dependencies**: None (build dependencies handled by Homebrew)

## Installation

### For Users

```bash
# From custom tap (initial)
brew tap emgeedata/tap
brew install rtbt

# From homebrew core (future)
brew install rtbt
```

### For Maintainers

#### Local Testing

1. **Test formula locally**:

   ```bash
   brew install --build-from-source ./rtbt.rb
   brew test rtbt
   ```

2. **Verify installation**:

   ```bash
   rtbt --help
   rtbt --list-palettes
   ```

3. **Test completions**:

   ```bash
   # Should work after shell restart
   rtbt --<TAB>
   ```

#### Custom Tap Setup

1. **Create tap repository**:

   ```bash
   # Create repository: homebrew-tap
   # Add rtbt.rb to repository root
   ```

2. **Users can then**:

   ```bash
   brew tap emgeedata/tap
   brew install rtbt
   ```

#### Homebrew Core Submission

1. **Meet core requirements**:
   - Notable project (GitHub stars, usage)
   - Stable release history
   - No GUI applications
   - Not a duplicate of existing formula

2. **Submit pull request**:

   ```bash
   # Fork homebrew/homebrew-core
   # Add formula to Formula/rtbt.rb
   # Submit PR following their guidelines
   ```

## Files Installed

```bash
/usr/local/bin/rtbt                                           # Main binary (Intel)
/opt/homebrew/bin/rtbt                                        # Main binary (Apple Silicon)
/usr/local/share/bash-completion/completions/rtbt            # Bash completion
/usr/local/share/zsh/site-functions/_rtbt                   # Zsh completion
/usr/local/share/fish/vendor_completions.d/rtbt.fish        # Fish completion
/usr/local/share/man/man1/rtbt.1                            # Manual page
```

## Testing

```bash
# Install from local formula
brew install --build-from-source ./rtbt.rb

# Run tests
brew test rtbt

# Verify functionality
rtbt --help
rtbt --version

# Test with sample image
rtbt -i /path/to/image.jpg -o output.png

# Uninstall
brew uninstall rtbt
```

## Formula Requirements

The formula follows [Homebrew guidelines](https://docs.brew.sh/Formula-Cookbook):

- ✅ Uses stable source URL and checksum
- ✅ Builds from source using cargo
- ✅ Installs shell completions properly
- ✅ Includes test block
- ✅ Proper license and description
- ✅ No unnecessary dependencies

## Troubleshooting

### Build Issues

- **Rust missing**: Homebrew installs rust automatically
- **Build timeout**: Large projects may need longer build times
- **Architecture issues**: Test on both Intel and Apple Silicon

### Installation Issues

- **Binary not found**: Check PATH includes Homebrew bin directory
- **Permission denied**: Homebrew handles permissions automatically
- **Completions missing**: May require shell restart

## Updating Formula

1. **New release**: Update version and sha256
2. **Test build**: `brew install --build-from-source ./rtbt.rb`
3. **Verify**: Run basic functionality tests
4. **Update tap**: Commit and push to tap repository

## Architecture Support

- ✅ **Intel (x86_64)**: Supported
- ✅ **Apple Silicon (arm64)**: Supported  
- ✅ **Universal Binary**: Rust builds appropriate architecture automatically

# Documentation

This directory contains documentation files for RustBucket.

## Contents

### Man Pages

- **`man/rtbt.1`** - Manual page for the `rtbt` command
  - Complete command reference with all options
  - Built-in palette documentation (17 themes)
  - Custom palette format and examples
  - Usage examples and performance notes

## Installing Documentation

### Man Page Installation

#### System-wide installation

```bash
# Copy to system man directory
sudo cp docs/man/rtbt.1 /usr/local/share/man/man1/
sudo mandb  # Update man database

# View the man page
man rtbt
```

#### User-local installation

```bash
# Create user man directory if needed
mkdir -p ~/.local/share/man/man1

# Copy man page
cp docs/man/rtbt.1 ~/.local/share/man/man1/

# Add to MANPATH if needed
export MANPATH="$HOME/.local/share/man:$MANPATH"

# View the man page
man rtbt
```

#### Temporary viewing

```bash
# View without installing
man ./docs/man/rtbt.1
```

## Package Integration

The man page is automatically included in package installations:

- **Arch Linux**: Installed to `/usr/share/man/man1/rtbt.1`
- **Homebrew**: Installed to appropriate man directory for the platform

## Updating Documentation

When updating the man page:

1. Edit `docs/man/rtbt.1` using standard man page format
2. Test with `man ./docs/man/rtbt.1`
3. Update package definitions to include the new documentation
4. Update version and date in the man page header

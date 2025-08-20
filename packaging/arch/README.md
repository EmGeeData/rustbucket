# Arch Linux Package (AUR)

This directory contains the PKGBUILD for distributing RustBucket via the Arch User Repository (AUR).

## Package Information

- **Package Name**: `rustbucket`
- **AUR Package Name**: `rustbucket`
- **Maintainer**: EmGeeData
- **Build Type**: Source build using cargo
- **Dependencies**: `rust`, `cargo` (build-time only)

## Installation

### For Users

```bash
# Using yay
yay -S rustbucket

# Using paru  
paru -S rustbucket

# Manual build
git clone https://aur.archlinux.org/rustbucket.git
cd rustbucket
makepkg -si
```

### For Maintainers

#### Initial AUR Submission

1. **Test locally**:

   ```bash
   makepkg -s
   pacman -U rustbucket-*.pkg.tar.zst
   ```

2. **Generate .SRCINFO**:

   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```

3. **Submit to AUR**:

   ```bash
   git clone ssh://aur@aur.archlinux.org/rustbucket.git
   cp PKGBUILD .SRCINFO rustbucket/
   cd rustbucket
   git add PKGBUILD .SRCINFO
   git commit -m "Initial commit"
   git push origin master
   ```

#### Updates

1. **Update PKGBUILD** with new version and checksums
2. **Test build**: `makepkg -s`
3. **Update .SRCINFO**: `makepkg --printsrcinfo > .SRCINFO`
4. **Commit and push**:

   ```bash
   git add PKGBUILD .SRCINFO
   git commit -m "Update to version X.Y.Z"
   git push
   ```

## Files Installed

```bash
/usr/bin/rtbt                                    # Main binary
/usr/share/bash-completion/completions/rtbt      # Bash completion
/usr/share/zsh/site-functions/_rtbt             # Zsh completion  
/usr/share/fish/vendor_completions.d/rtbt.fish # Fish completion
/usr/share/man/man1/rtbt.1                      # Manual page
```

## Testing

```bash
# Build package
makepkg -s

# Install and test
sudo pacman -U rustbucket-*.pkg.tar.zst
rtbt --help
rtbt --list-palettes

# Test shell completion
rtbt --<TAB>

# Remove package
sudo pacman -R rustbucket
```

## Troubleshooting

### Build Failures

- **Rust/Cargo missing**: Install `rust` package
- **Network issues**: Check internet connection for dependency downloads
- **Checksum mismatch**: Update pkgver and checksums in PKGBUILD

### Runtime Issues

- **Command not found**: Ensure `/usr/bin` is in PATH
- **Completions not working**: Re-source shell configuration

## AUR Guidelines

This package follows [AUR submission guidelines](https://wiki.archlinux.org/title/AUR_submission_guidelines):

- ✅ Unique package name
- ✅ Correct license specification
- ✅ Proper dependencies
- ✅ Standard installation paths
- ✅ Clean package removal

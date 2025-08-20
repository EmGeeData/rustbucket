# Shell Completions for rtbt

This directory contains shell completion files for the `rtbt` command. These completions provide tab-completion for all command-line options, arguments, and built-in palette names.

## Installation

### Bash

Add to your `~/.bashrc` or `~/.bash_profile`:

```bash
# Load rtbt completions
source /path/to/completions/bash/rtbt.bash
```

Or copy to your system's bash completion directory:

```bash
# On most Linux distributions:
sudo cp bash/rtbt.bash /etc/bash_completion.d/

# On macOS with Homebrew:
cp bash/rtbt.bash $(brew --prefix)/etc/bash_completion.d/
```

### Zsh

Add to your `~/.zshrc`:

```zsh
# Load rtbt completions
fpath=(/path/to/completions/zsh $fpath)
autoload -Uz compinit && compinit
```

Or copy to your system's zsh completion directory:

```bash
# On most systems:
sudo cp zsh/_rtbt /usr/share/zsh/site-functions/

# On macOS with Homebrew:
cp zsh/_rtbt $(brew --prefix)/share/zsh/site-functions/
```

### Fish

Copy to fish's completions directory:

```bash
# User-specific installation:
cp fish/rtbt.fish ~/.config/fish/completions/

# System-wide installation:
sudo cp fish/rtbt.fish /usr/share/fish/completions/
```

### PowerShell

Add to your PowerShell profile (`$PROFILE`):

```powershell
# Load rtbt completions
. "C:\path\to\completions\powershell\_rtbt.ps1"
```

Or to find your profile location:

```powershell
# Check if profile exists
Test-Path $PROFILE

# Create profile if it doesn't exist
if (!(Test-Path $PROFILE)) { New-Item -Type File -Path $PROFILE -Force }

# Edit profile
notepad $PROFILE
```

## Features

The completions provide:

- **Command options**: All flags like `--palette`, `--blur`, `--benchmark`
- **File paths**: Tab completion for input/output file paths
- **Built-in palettes**: Completion for palette names (nord, dracula, gruvbox, etc.)
- **Help text**: Brief descriptions for each option

## Examples

After installation, you can use tab completion like:

```bash
rtbt --p<TAB>          # Completes to --palette
rtbt --palette <TAB>   # Shows available palette names
rtbt -i <TAB>          # File completion for input files
rtbt --list-<TAB>      # Completes to --list-palettes
```

## Regenerating Completions

If you modify the CLI interface, regenerate completions with:

```bash
cargo run --bin generate-completions
```

This will update all completion files in this directory.

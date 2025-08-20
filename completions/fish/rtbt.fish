# Fish completion for rtbt
complete -c rtbt -f

# Options
complete -c rtbt -s i -l img -d "Input image path" -r -F
complete -c rtbt -s o -l out -d "Output image path" -r -F
complete -c rtbt -s p -l palette -d "Color palette to use" -x -a "nord dracula gruvbox monokai solarized catppuccin tokyo oceanic palenight onedark vim gotham challenger molokai sonokai serenade vaporwave"
complete -c rtbt -s c -l colors -d "Specific colors to use from palette" -x -a "red green blue yellow orange purple white black"
complete -c rtbt -s b -l blur -d "Apply Gaussian blur to the final result"
complete -c rtbt -s q -l quiet -d "Quiet mode - suppress output"
complete -c rtbt -l no-avg -d "Disable average pixels optimization algorithm"
complete -c rtbt -l pixels-area -d "Pixel area for average color calculation" -x -a "1,1 2,2 3,3 4,4 5,5 2,3 3,2"
complete -c rtbt -l benchmark -d "Run performance benchmarks and show optimization suggestions"
complete -c rtbt -l list-palettes -d "List all available built-in and user palettes"
complete -c rtbt -l create-palette -d "Create a skeleton palette file" -r -F
complete -c rtbt -l export-palette -d "Export a built-in palette to TOML format" -x
complete -c rtbt -s h -l help -d "Show help information"
complete -c rtbt -s V -l version -d "Show version information"

# Enhanced completions for specific options
# Input files - only suggest image files
complete -c rtbt -s i -l img -k -x -a "(__fish_complete_suffix .jpg .jpeg .png .JPG .JPEG .PNG)"

# Output files - suggest .png extension
complete -c rtbt -s o -l out -k -x -a "(commandline -ct).png"

# Palette completions with descriptions
complete -c rtbt -s p -l palette -k -x -a "
nord	'Arctic, north-bluish clean palette'
dracula	'Dark theme with vibrant accent colors'
gruvbox	'Retro groove warm color scheme'
monokai	'Popular dark coding theme'
solarized	'Precision engineered color palette'
catppuccin	'Soothing pastel theme'
tokyo	'Clean Tokyo Night inspired theme'
oceanic	'Deep ocean blue tones'
palenight	'Elegant dark purple theme'
onedark	'Atom\'s One Dark theme'
vim	'Classic Vim editor colors'
gotham	'Dark, Batman-inspired theme'
challenger	'High-contrast dark theme'
molokai	'Molokai terminal theme'
sonokai	'High-contrast color scheme'
serenade	'Calm, balanced color palette'
vaporwave	'Retro synthwave aesthetic'
"

# TOML files for custom palettes
complete -c rtbt -s p -l palette -k -x -a "(__fish_complete_suffix .toml)"

# Color examples with descriptions
complete -c rtbt -s c -l colors -k -x -a "
red\t'Primary red color'
green\t'Primary green color'
blue\t'Primary blue color'
yellow\t'Primary yellow color'
orange\t'Orange color'
purple\t'Purple color'
white\t'White color'
black\t'Black color'
"

# Pixel area examples with descriptions
complete -c rtbt -l pixels-area -k -x -a "
1,1\t'Single pixel'
2,2\t'2x2 pixel area'
3,3\t'3x3 pixel area'
4,4\t'4x4 pixel area'
5,5\t'5x5 pixel area'
2,3\t'2x3 pixel area'
3,2\t'3x2 pixel area'
"

# Create palette - suggest .toml extension
complete -c rtbt -l create-palette -k -x -a "(commandline -ct).toml"

# Export palette completions
function __rtbt_export_palette_completions
    set -l tokens (commandline -opc)
    set -l token_count (count $tokens)
    
    # Check if we're after --export-palette
    if contains -- --export-palette $tokens
        # Find position of --export-palette
        set -l export_index (contains -i -- --export-palette $tokens)
        set -l arg_position (math $token_count - $export_index)
        
        if test $arg_position -eq 1
            # First argument: palette name
            echo "nord	Arctic, north-bluish clean palette"
            echo "dracula	Dark theme with vibrant accent colors"
            echo "gruvbox	Retro groove warm color scheme"
            echo "monokai	Popular dark coding theme"
            echo "solarized	Precision engineered color palette"
            echo "catppuccin	Soothing pastel theme"
            echo "tokyo	Clean Tokyo Night inspired theme"
            echo "oceanic	Deep ocean blue tones"
            echo "palenight	Elegant dark purple theme"
            echo "onedark	Atom's One Dark theme"
            echo "vim	Classic Vim editor colors"
            echo "gotham	Dark, Batman-inspired theme"
            echo "challenger	High-contrast dark theme"
            echo "molokai	Molokai terminal theme"
            echo "sonokai	High-contrast color scheme"
            echo "serenade	Calm, balanced color palette"
            echo "vaporwave	Retro synthwave aesthetic"
        else if test $arg_position -eq 2
            # Second argument: output file (.toml)
            set -l current_token (commandline -ct)
            if not string match -q "*.toml" $current_token
                echo "$current_token.toml"
            end
        end
    end
end

complete -c rtbt -l export-palette -k -x -a "(__rtbt_export_palette_completions)"

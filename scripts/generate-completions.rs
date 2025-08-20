#!/usr/bin/env rust-script

//! Generate enhanced shell completion files for rtbt
//!
//! This script generates sophisticated shell completions with palette names,
//! smart file filtering, and context-aware suggestions for all major shells.
//!
//! Usage: cargo run --bin generate-completions

use std::fs;
use std::path::PathBuf;

/// Built-in palette names - keep this in sync with src/palette/builtin.rs
const PALETTE_NAMES: &[&str] = &[
    "nord",
    "dracula",
    "gruvbox",
    "monokai",
    "solarized",
    "catppuccin",
    "tokyo",
    "oceanic",
    "palenight",
    "onedark",
    "vim",
    "gotham",
    "challenger",
    "molokai",
    "sonokai",
    "serenade",
    "vaporwave",
];

/// Generate enhanced Bash completion
fn generate_bash_completion() -> String {
    let palettes = PALETTE_NAMES.join(" ");

    format!(
        r#"_rtbt() {{
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${{BASH_VERSINFO[0]}}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${{COMP_WORDS[COMP_CWORD]}}"
    fi
    prev="$3"
    cmd=""
    opts=""

    # Built-in palette names
    local palettes="{palettes}"

    for i in "${{COMP_WORDS[@]:0:COMP_CWORD}}"
    do
        case "${{cmd}},${{i}}" in
            ",$1")
                cmd="rtbt"
                ;;
            *)
                ;;
        esac
    done

    case "${{cmd}}" in
        rtbt)
            opts="-i -o -p -c -b -q -h -V --img --out --palette --colors --blur --quiet --no-avg --pixels-area --benchmark --list-palettes --create-palette --export-palette --help --version"
            if [[ ${{cur}} == -* || ${{COMP_CWORD}} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${{opts}}" -- "${{cur}}") )
                return 0
            fi
            case "${{prev}}" in
                --img|-i)
                    # Complete image files
                    COMPREPLY=($(compgen -f -X "!*.@(jpg|jpeg|png|JPG|JPEG|PNG)" -- "${{cur}}"))
                    [[ ${{#COMPREPLY[@]}} -eq 0 ]] && COMPREPLY=($(compgen -f -- "${{cur}}"))
                    return 0
                    ;;
                --out|-o)
                    # Complete with .png extension suggestion
                    if [[ ${{cur}} != *.* ]]; then
                        COMPREPLY=($(compgen -W "${{cur}}.png" -- "${{cur}}"))
                    fi
                    COMPREPLY+=($(compgen -f -- "${{cur}}"))
                    return 0
                    ;;
                --palette|-p)
                    # Complete with built-in palette names and TOML files
                    COMPREPLY=($(compgen -W "${{palettes}}" -- "${{cur}}"))
                    COMPREPLY+=($(compgen -f -X "!*.toml" -- "${{cur}}"))
                    return 0
                    ;;
                --colors|-c)
                    # Provide common color examples
                    local color_examples="red green blue yellow orange purple white black"
                    COMPREPLY=($(compgen -W "${{color_examples}}" -- "${{cur}}"))
                    return 0
                    ;;
                --pixels-area)
                    # Provide common pixel area examples
                    local area_examples="1,1 2,2 3,3 4,4 5,5 2,3 3,2"
                    COMPREPLY=($(compgen -W "${{area_examples}}" -- "${{cur}}"))
                    return 0
                    ;;
                --create-palette)
                    # Suggest .toml extension
                    if [[ ${{cur}} != *.* ]]; then
                        COMPREPLY=($(compgen -W "${{cur}}.toml" -- "${{cur}}"))
                    fi
                    COMPREPLY+=($(compgen -f -- "${{cur}}"))
                    return 0
                    ;;
                --export-palette)
                    # Check if we're completing the first or second argument
                    local export_args=0
                    for (( i=1; i<COMP_CWORD; i++ )); do
                        if [[ "${{COMP_WORDS[$i]}}" == "--export-palette" ]]; then
                            export_args=$((COMP_CWORD - i - 1))
                            break
                        fi
                    done
                    
                    if [[ $export_args -eq 1 ]]; then
                        # First argument: palette name
                        COMPREPLY=($(compgen -W "${{palettes}}" -- "${{cur}}"))
                    elif [[ $export_args -eq 2 ]]; then
                        # Second argument: output file
                        if [[ ${{cur}} != *.* ]]; then
                            COMPREPLY=($(compgen -W "${{cur}}.toml" -- "${{cur}}"))
                        fi
                        COMPREPLY+=($(compgen -f -- "${{cur}}"))
                    fi
                    return 0
                    ;;
                *)
                    # Default file completion
                    COMPREPLY=($(compgen -f -- "${{cur}}"))
                    ;;
            esac
            ;;
    esac
}}

# Enable completion
if [[ "${{BASH_VERSINFO[0]}}" -eq 4 && "${{BASH_VERSINFO[1]}}" -ge 4 || "${{BASH_VERSINFO[0]}}" -gt 4 ]]; then
    complete -F _rtbt -o nosort -o bashdefault -o default rtbt
else
    complete -F _rtbt -o bashdefault -o default rtbt
fi
"#,
        palettes = palettes
    )
}

/// Generate enhanced Fish completion
fn generate_fish_completion() -> String {
    let palette_list = PALETTE_NAMES.join(" ");
    let palette_descriptions = PALETTE_NAMES
        .iter()
        .map(|&p| match p {
            "nord" => format!("{p}\t'Arctic, north-bluish clean palette'"),
            "dracula" => format!("{p}\t'Dark theme with vibrant accent colors'"),
            "gruvbox" => format!("{p}\t'Retro groove warm color scheme'"),
            "monokai" => format!("{p}\t'Popular dark coding theme'"),
            "solarized" => format!("{p}\t'Precision engineered color palette'"),
            "catppuccin" => format!("{p}\t'Soothing pastel theme'"),
            "tokyo" => format!("{p}\t'Clean Tokyo Night inspired theme'"),
            "oceanic" => format!("{p}\t'Deep ocean blue tones'"),
            "palenight" => format!("{p}\t'Elegant dark purple theme'"),
            "onedark" => format!("{p}\t'Atom\\'s One Dark theme'"),
            "vim" => format!("{p}\t'Classic Vim editor colors'"),
            "gotham" => format!("{p}\t'Dark, Batman-inspired theme'"),
            "challenger" => format!("{p}\t'High-contrast dark theme'"),
            "molokai" => format!("{p}\t'Molokai terminal theme'"),
            "sonokai" => format!("{p}\t'High-contrast color scheme'"),
            "serenade" => format!("{p}\t'Calm, balanced color palette'"),
            "vaporwave" => format!("{p}\t'Retro synthwave aesthetic'"),
            _ => format!("{p}\t'{p} palette'"),
        })
        .collect::<Vec<_>>()
        .join("\n");

    let export_palette_descriptions = PALETTE_NAMES
        .iter()
        .map(|&p| match p {
            "nord" => format!("echo \"{p}\tArctic, north-bluish clean palette\""),
            "dracula" => format!("echo \"{p}\tDark theme with vibrant accent colors\""),
            "gruvbox" => format!("echo \"{p}\tRetro groove warm color scheme\""),
            "monokai" => format!("echo \"{p}\tPopular dark coding theme\""),
            "solarized" => format!("echo \"{p}\tPrecision engineered color palette\""),
            "catppuccin" => format!("echo \"{p}\tSoothing pastel theme\""),
            "tokyo" => format!("echo \"{p}\tClean Tokyo Night inspired theme\""),
            "oceanic" => format!("echo \"{p}\tDeep ocean blue tones\""),
            "palenight" => format!("echo \"{p}\tElegant dark purple theme\""),
            "onedark" => format!("echo \"{p}\tAtom's One Dark theme\""),
            "vim" => format!("echo \"{p}\tClassic Vim editor colors\""),
            "gotham" => format!("echo \"{p}\tDark, Batman-inspired theme\""),
            "challenger" => format!("echo \"{p}\tHigh-contrast dark theme\""),
            "molokai" => format!("echo \"{p}\tMolokai terminal theme\""),
            "sonokai" => format!("echo \"{p}\tHigh-contrast color scheme\""),
            "serenade" => format!("echo \"{p}\tCalm, balanced color palette\""),
            "vaporwave" => format!("echo \"{p}\tRetro synthwave aesthetic\""),
            _ => format!("echo \"{p}\t{p} palette\""),
        })
        .collect::<Vec<_>>()
        .join("\n            ");

    format!(
        r#"# Fish completion for rtbt
complete -c rtbt -f

# Options
complete -c rtbt -s i -l img -d "Input image path" -r -F
complete -c rtbt -s o -l out -d "Output image path" -r -F
complete -c rtbt -s p -l palette -d "Color palette to use" -x -a "{palette_list}"
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
{palette_descriptions}
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
            {export_palette_descriptions}
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
"#,
        palette_list = palette_list,
        palette_descriptions = palette_descriptions,
        export_palette_descriptions = export_palette_descriptions
    )
}

/// Generate enhanced Zsh completion
fn generate_zsh_completion() -> String {
    let palette_descriptions = PALETTE_NAMES
        .iter()
        .map(|&p| match p {
            "nord" => format!("        '{p}:Arctic, north-bluish clean palette'"),
            "dracula" => format!("        '{p}:Dark theme with vibrant accent colors'"),
            "gruvbox" => format!("        '{p}:Retro groove warm color scheme'"),
            "monokai" => format!("        '{p}:Popular dark coding theme'"),
            "solarized" => format!("        '{p}:Precision engineered color palette'"),
            "catppuccin" => format!("        '{p}:Soothing pastel theme'"),
            "tokyo" => format!("        '{p}:Clean Tokyo Night inspired theme'"),
            "oceanic" => format!("        '{p}:Deep ocean blue tones'"),
            "palenight" => format!("        '{p}:Elegant dark purple theme'"),
            "onedark" => format!("        '{p}:Atom'\\''s One Dark theme'"),
            "vim" => format!("        '{p}:Classic Vim editor colors'"),
            "gotham" => format!("        '{p}:Dark, Batman-inspired theme'"),
            "challenger" => format!("        '{p}:High-contrast dark theme'"),
            "molokai" => format!("        '{p}:Molokai terminal theme'"),
            "sonokai" => format!("        '{p}:High-contrast color scheme'"),
            "serenade" => format!("        '{p}:Calm, balanced color palette'"),
            "vaporwave" => format!("        '{p}:Retro synthwave aesthetic'"),
            _ => format!("        '{p}:{p} palette'"),
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"#compdef rtbt

autoload -U is-at-least

_rtbt() {{
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    # Built-in palette names
    local -a palettes
    palettes=(
{palette_descriptions}
    )

    # Pixel area examples
    local -a pixel_areas
    pixel_areas=(
        '1,1:Single pixel'
        '2,2:2x2 pixel area'
        '3,3:3x3 pixel area'
        '4,4:4x4 pixel area'
        '5,5:5x5 pixel area'
        '2,3:2x3 pixel area'
        '3,2:3x2 pixel area'
    )

    local context curcontext="$curcontext" state line
    _arguments "${{_arguments_options[@]}}" \
        '(-i --img)'{{-i,--img}}'[Input image path]:input file:_files -g "*.{{jpg,jpeg,png,JPG,JPEG,PNG}}"' \
        '(-o --out)'{{-o,--out}}'[Output image path]:output file:_files' \
        '(-p --palette)'{{-p,--palette}}'[Color palette to use]:palette:->palette' \
        '(-c --colors)'{{-c,--colors}}'[Specific colors to use from palette]:colors:->colors' \
        '(-b --blur)'{{-b,--blur}}'[Apply Gaussian blur to the final result]' \
        '(-q --quiet)'{{-q,--quiet}}'[Quiet mode - suppress output]' \
        '--no-avg[Disable average pixels optimization algorithm]' \
        '--pixels-area[Pixel area for average color calculation]:area:->pixels_area' \
        '--benchmark[Run performance benchmarks and show optimization suggestions]' \
        '--list-palettes[List all available built-in and user palettes]' \
        '--create-palette[Create a skeleton palette file]:output file:_files -g "*.toml"' \
        '--export-palette[Export a built-in palette to TOML format]:palette and output:->export_palette' \
        '(-h --help)'{{-h,--help}}'[Show help information]' \
        '(-V --version)'{{-V,--version}}'[Show version information]' \
        && ret=0

    case $state in
        palette)
            _describe -t palettes 'built-in palettes' palettes
            _files -g "*.toml" && ret=0
            ;;
        colors)
            local -a color_examples
            color_examples=(
                'red:Primary red color'
                'green:Primary green color'
                'blue:Primary blue color'
                'yellow:Primary yellow color'
                'orange:Orange color'
                'purple:Purple color'
                'white:White color'
                'black:Black color'
            )
            _describe -t colors 'color examples' color_examples && ret=0
            ;;
        pixels_area)
            _describe -t pixel_areas 'pixel area examples' pixel_areas && ret=0
            ;;
        export_palette)
            if (( CURRENT == 3 )); then
                # First argument: palette name
                _describe -t palettes 'built-in palettes' palettes && ret=0
            elif (( CURRENT == 4 )); then
                # Second argument: output file
                _files -g "*.toml" && ret=0
            fi
            ;;
    esac

    return ret
}}

(( $+functions[_rtbt_commands] )) ||
_rtbt_commands() {{
    local commands; commands=()
    _describe -t commands 'rtbt commands' commands "$@"
}}

if [[ "$funcstack[1]" == "_rtbt" ]]; then
    _rtbt "$@"
else
    compdef _rtbt rtbt
fi
"#,
        palette_descriptions = palette_descriptions
    )
}

/// Generate enhanced PowerShell completion
fn generate_powershell_completion() -> String {
    let palette_entries = PALETTE_NAMES
        .iter()
        .map(|&p| match p {
            "nord" => {
                "        'nord' = 'Arctic, north-bluish clean palette (16 colors)'".to_string()
            }
            "dracula" => "        'dracula' = 'Dark theme with vibrant accent colors (11 colors)'"
                .to_string(),
            "gruvbox" => {
                "        'gruvbox' = 'Retro groove warm color scheme (17 colors)'".to_string()
            }
            "monokai" => "        'monokai' = 'Popular dark coding theme (9 colors)'".to_string(),
            "solarized" => {
                "        'solarized' = 'Precision engineered color palette (16 colors)'".to_string()
            }
            "catppuccin" => {
                "        'catppuccin' = 'Soothing pastel theme (16 colors)'".to_string()
            }
            "tokyo" => {
                "        'tokyo' = 'Clean Tokyo Night inspired theme (29 colors)'".to_string()
            }
            "oceanic" => "        'oceanic' = 'Deep ocean blue tones (16 colors)'".to_string(),
            "palenight" => {
                "        'palenight' = 'Elegant dark purple theme (10 colors)'".to_string()
            }
            "onedark" => "        'onedark' = 'Atom''s One Dark theme (8 colors)'".to_string(),
            "vim" => "        'vim' = 'Classic Vim editor colors (16 colors)'".to_string(),
            "gotham" => "        'gotham' = 'Dark, Batman-inspired theme (16 colors)'".to_string(),
            "challenger" => {
                "        'challenger' = 'High-contrast dark theme (10 colors)'".to_string()
            }
            "molokai" => "        'molokai' = 'Molokai terminal theme (8 colors)'".to_string(),
            "sonokai" => "        'sonokai' = 'High-contrast color scheme (12 colors)'".to_string(),
            "serenade" => {
                "        'serenade' = 'Calm, balanced color palette (10 colors)'".to_string()
            }
            "vaporwave" => {
                "        'vaporwave' = 'Retro synthwave aesthetic (10 colors)'".to_string()
            }
            _ => format!("        '{p}' = '{p} palette'"),
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'rtbt' -ScriptBlock {{
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'rtbt'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {{
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {{
                break
        }}
        $element.Value
    }}) -join ';'

    # Built-in palette names with descriptions
    $palettes = @{{
{palette_entries}
    }}

    # Color examples
    $colors = @{{
        'red' = 'Primary red color'
        'green' = 'Primary green color'
        'blue' = 'Primary blue color'
        'yellow' = 'Primary yellow color'
        'orange' = 'Orange color'
        'purple' = 'Purple color'
        'white' = 'White color'
        'black' = 'Black color'
    }}

    # Pixel area examples
    $pixelAreas = @{{
        '1,1' = 'Single pixel'
        '2,2' = '2x2 pixel area'
        '3,3' = '3x3 pixel area'
        '4,4' = '4x4 pixel area'
        '5,5' = '5x5 pixel area'
        '2,3' = '2x3 pixel area'
        '3,2' = '3x2 pixel area'
    }}

    $completions = @()

    # Check the previous argument to provide context-specific completions
    $previousArg = $null
    if ($commandElements.Count -gt 1) {{
        $previousArg = $commandElements[-2].Value
    }}

    switch ($previousArg) {{
        {{ $_ -in @('-i', '--img') }} {{
            # Complete image files
            $imageExtensions = @('*.jpg', '*.jpeg', '*.png', '*.JPG', '*.JPEG', '*.PNG')
            foreach ($ext in $imageExtensions) {{
                $files = Get-ChildItem -Path $ext -ErrorAction SilentlyContinue
                foreach ($file in $files) {{
                    $completions += [CompletionResult]::new($file.Name, $file.Name, 'ParameterValue', "Image file: $($file.Name)")
                }}
            }}
        }}
        
        {{ $_ -in @('-o', '--out') }} {{
            # Suggest .png extension if no extension provided
            if ($wordToComplete -and -not $wordToComplete.Contains('.')) {{
                $completions += [CompletionResult]::new("$wordToComplete.png", "$wordToComplete.png", 'ParameterValue', 'Output PNG file')
            }}
            # Also complete existing files
            $files = Get-ChildItem -ErrorAction SilentlyContinue
            foreach ($file in $files) {{
                if ($file.Name -like "*$wordToComplete*") {{
                    $completions += [CompletionResult]::new($file.Name, $file.Name, 'ParameterValue', "File: $($file.Name)")
                }}
            }}
        }}
        
        {{ $_ -in @('-p', '--palette') }} {{
            # Complete palette names
            foreach ($palette in $palettes.GetEnumerator()) {{
                if ($palette.Key -like "*$wordToComplete*") {{
                    $completions += [CompletionResult]::new($palette.Key, $palette.Key, 'ParameterValue', $palette.Value)
                }}
            }}
            # Also complete TOML files
            $tomlFiles = Get-ChildItem -Path '*.toml' -ErrorAction SilentlyContinue
            foreach ($file in $tomlFiles) {{
                if ($file.Name -like "*$wordToComplete*") {{
                    $completions += [CompletionResult]::new($file.Name, $file.Name, 'ParameterValue', "Custom palette: $($file.Name)")
                }}
            }}
        }}
        
        {{ $_ -in @('-c', '--colors') }} {{
            # Complete color names
            foreach ($color in $colors.GetEnumerator()) {{
                if ($color.Key -like "*$wordToComplete*") {{
                    $completions += [CompletionResult]::new($color.Key, $color.Key, 'ParameterValue', $color.Value)
                }}
            }}
        }}
        
        '--pixels-area' {{
            # Complete pixel area examples
            foreach ($area in $pixelAreas.GetEnumerator()) {{
                if ($area.Key -like "*$wordToComplete*") {{
                    $completions += [CompletionResult]::new($area.Key, $area.Key, 'ParameterValue', $area.Value)
                }}
            }}
        }}
        
        '--create-palette' {{
            # Suggest .toml extension
            if ($wordToComplete -and -not $wordToComplete.Contains('.')) {{
                $completions += [CompletionResult]::new("$wordToComplete.toml", "$wordToComplete.toml", 'ParameterValue', 'TOML palette file')
            }}
        }}
        
        '--export-palette' {{
            # For export-palette, check which argument we're completing
            $exportIndex = -1
            for ($i = 0; $i -lt $commandElements.Count; $i++) {{
                if ($commandElements[$i].Value -eq '--export-palette') {{
                    $exportIndex = $i
                    break
                }}
            }}
            
            if ($exportIndex -ge 0) {{
                $argPosition = $commandElements.Count - $exportIndex - 1
                if ($argPosition -eq 1) {{
                    # First argument: palette name
                    foreach ($palette in $palettes.GetEnumerator()) {{
                        if ($palette.Key -like "*$wordToComplete*") {{
                            $completions += [CompletionResult]::new($palette.Key, $palette.Key, 'ParameterValue', $palette.Value)
                        }}
                    }}
                }} elseif ($argPosition -eq 2) {{
                    # Second argument: output file
                    if ($wordToComplete -and -not $wordToComplete.Contains('.')) {{
                        $completions += [CompletionResult]::new("$wordToComplete.toml", "$wordToComplete.toml", 'ParameterValue', 'Output TOML file')
                    }}
                }}
            }}
        }}
    }}

    # If no specific completions found, provide general options
    if ($completions.Count -eq 0) {{
        $options = @(
            @('-i', '--img', 'Input image path'),
            @('-o', '--out', 'Output image path'),
            @('-p', '--palette', 'Color palette to use'),
            @('-c', '--colors', 'Specific colors to use from palette'),
            @('-b', '--blur', 'Apply Gaussian blur to the final result'),
            @('-q', '--quiet', 'Quiet mode - suppress output'),
            @('--no-avg', $null, 'Disable average pixels optimization algorithm'),
            @('--pixels-area', $null, 'Pixel area for average color calculation'),
            @('--benchmark', $null, 'Run performance benchmarks and show optimization suggestions'),
            @('--list-palettes', $null, 'List all available built-in and user palettes'),
            @('--create-palette', $null, 'Create a skeleton palette file'),
            @('--export-palette', $null, 'Export a built-in palette to TOML format'),
            @('-h', '--help', 'Show help information'),
            @('-V', '--version', 'Show version information')
        )
        
        foreach ($option in $options) {{
            $short, $long, $description = $option
            if ($short -and $short -like "*$wordToComplete*") {{
                $completions += [CompletionResult]::new($short, $short, 'ParameterName', $description)
            }}
            if ($long -and $long -like "*$wordToComplete*") {{
                $completions += [CompletionResult]::new($long, $long, 'ParameterName', $description)
            }}
        }}
    }}

    $completions
}}
"#,
        palette_entries = palette_entries
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create completions directory structure
    let base_completions_dir = PathBuf::from("completions");
    fs::create_dir_all(&base_completions_dir)?;

    // Generate enhanced completions for each shell
    let completions = [
        ("bash", "rtbt.bash", generate_bash_completion()),
        ("fish", "rtbt.fish", generate_fish_completion()),
        ("zsh", "_rtbt", generate_zsh_completion()),
        ("powershell", "_rtbt.ps1", generate_powershell_completion()),
    ];

    for (shell_name, filename, content) in completions {
        let shell_dir = base_completions_dir.join(shell_name);
        fs::create_dir_all(&shell_dir)?;

        let file_path = shell_dir.join(filename);
        fs::write(&file_path, content)?;

        println!(
            "Generated enhanced {} completion: {}",
            shell_name,
            file_path.display()
        );
    }

    println!(
        "\nEnhanced completion files generated in shell-specific directories under: {}",
        base_completions_dir.display()
    );
    println!("Features include:");
    println!(
        "  • Built-in palette name completion for all {} palettes",
        PALETTE_NAMES.len()
    );
    println!("  • Smart image file filtering (jpg, jpeg, png)");
    println!("  • Context-aware suggestions for --export-palette");
    println!("  • TOML file completion for custom palettes");
    println!("  • Color and pixel area examples");
    println!("\nInstall instructions can be found in completions/README.md");

    Ok(())
}

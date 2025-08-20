using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'rtbt' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'rtbt'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    # Built-in palette names with descriptions
    $palettes = @{
        'nord' = 'Arctic, north-bluish clean palette (16 colors)'
        'dracula' = 'Dark theme with vibrant accent colors (11 colors)'
        'gruvbox' = 'Retro groove warm color scheme (17 colors)'
        'monokai' = 'Popular dark coding theme (9 colors)'
        'solarized' = 'Precision engineered color palette (16 colors)'
        'catppuccin' = 'Soothing pastel theme (16 colors)'
        'tokyo' = 'Clean Tokyo Night inspired theme (29 colors)'
        'oceanic' = 'Deep ocean blue tones (16 colors)'
        'palenight' = 'Elegant dark purple theme (10 colors)'
        'onedark' = 'Atom''s One Dark theme (8 colors)'
        'vim' = 'Classic Vim editor colors (16 colors)'
        'gotham' = 'Dark, Batman-inspired theme (16 colors)'
        'challenger' = 'High-contrast dark theme (10 colors)'
        'molokai' = 'Molokai terminal theme (8 colors)'
        'sonokai' = 'High-contrast color scheme (12 colors)'
        'serenade' = 'Calm, balanced color palette (10 colors)'
        'vaporwave' = 'Retro synthwave aesthetic (10 colors)'
    }

    # Color examples
    $colors = @{
        'red' = 'Primary red color'
        'green' = 'Primary green color'
        'blue' = 'Primary blue color'
        'yellow' = 'Primary yellow color'
        'orange' = 'Orange color'
        'purple' = 'Purple color'
        'white' = 'White color'
        'black' = 'Black color'
    }

    # Pixel area examples
    $pixelAreas = @{
        '1,1' = 'Single pixel'
        '2,2' = '2x2 pixel area'
        '3,3' = '3x3 pixel area'
        '4,4' = '4x4 pixel area'
        '5,5' = '5x5 pixel area'
        '2,3' = '2x3 pixel area'
        '3,2' = '3x2 pixel area'
    }

    $completions = @()

    # Check the previous argument to provide context-specific completions
    $previousArg = $null
    if ($commandElements.Count -gt 1) {
        $previousArg = $commandElements[-2].Value
    }

    switch ($previousArg) {
        { $_ -in @('-i', '--img') } {
            # Complete image files
            $imageExtensions = @('*.jpg', '*.jpeg', '*.png', '*.JPG', '*.JPEG', '*.PNG')
            foreach ($ext in $imageExtensions) {
                $files = Get-ChildItem -Path $ext -ErrorAction SilentlyContinue
                foreach ($file in $files) {
                    $completions += [CompletionResult]::new($file.Name, $file.Name, 'ParameterValue', "Image file: $($file.Name)")
                }
            }
        }
        
        { $_ -in @('-o', '--out') } {
            # Suggest .png extension if no extension provided
            if ($wordToComplete -and -not $wordToComplete.Contains('.')) {
                $completions += [CompletionResult]::new("$wordToComplete.png", "$wordToComplete.png", 'ParameterValue', 'Output PNG file')
            }
            # Also complete existing files
            $files = Get-ChildItem -ErrorAction SilentlyContinue
            foreach ($file in $files) {
                if ($file.Name -like "*$wordToComplete*") {
                    $completions += [CompletionResult]::new($file.Name, $file.Name, 'ParameterValue', "File: $($file.Name)")
                }
            }
        }
        
        { $_ -in @('-p', '--palette') } {
            # Complete palette names
            foreach ($palette in $palettes.GetEnumerator()) {
                if ($palette.Key -like "*$wordToComplete*") {
                    $completions += [CompletionResult]::new($palette.Key, $palette.Key, 'ParameterValue', $palette.Value)
                }
            }
            # Also complete TOML files
            $tomlFiles = Get-ChildItem -Path '*.toml' -ErrorAction SilentlyContinue
            foreach ($file in $tomlFiles) {
                if ($file.Name -like "*$wordToComplete*") {
                    $completions += [CompletionResult]::new($file.Name, $file.Name, 'ParameterValue', "Custom palette: $($file.Name)")
                }
            }
        }
        
        { $_ -in @('-c', '--colors') } {
            # Complete color names
            foreach ($color in $colors.GetEnumerator()) {
                if ($color.Key -like "*$wordToComplete*") {
                    $completions += [CompletionResult]::new($color.Key, $color.Key, 'ParameterValue', $color.Value)
                }
            }
        }
        
        '--pixels-area' {
            # Complete pixel area examples
            foreach ($area in $pixelAreas.GetEnumerator()) {
                if ($area.Key -like "*$wordToComplete*") {
                    $completions += [CompletionResult]::new($area.Key, $area.Key, 'ParameterValue', $area.Value)
                }
            }
        }
        
        '--create-palette' {
            # Suggest .toml extension
            if ($wordToComplete -and -not $wordToComplete.Contains('.')) {
                $completions += [CompletionResult]::new("$wordToComplete.toml", "$wordToComplete.toml", 'ParameterValue', 'TOML palette file')
            }
        }
        
        '--export-palette' {
            # For export-palette, check which argument we're completing
            $exportIndex = -1
            for ($i = 0; $i -lt $commandElements.Count; $i++) {
                if ($commandElements[$i].Value -eq '--export-palette') {
                    $exportIndex = $i
                    break
                }
            }
            
            if ($exportIndex -ge 0) {
                $argPosition = $commandElements.Count - $exportIndex - 1
                if ($argPosition -eq 1) {
                    # First argument: palette name
                    foreach ($palette in $palettes.GetEnumerator()) {
                        if ($palette.Key -like "*$wordToComplete*") {
                            $completions += [CompletionResult]::new($palette.Key, $palette.Key, 'ParameterValue', $palette.Value)
                        }
                    }
                } elseif ($argPosition -eq 2) {
                    # Second argument: output file
                    if ($wordToComplete -and -not $wordToComplete.Contains('.')) {
                        $completions += [CompletionResult]::new("$wordToComplete.toml", "$wordToComplete.toml", 'ParameterValue', 'Output TOML file')
                    }
                }
            }
        }
    }

    # If no specific completions found, provide general options
    if ($completions.Count -eq 0) {
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
        
        foreach ($option in $options) {
            $short, $long, $description = $option
            if ($short -and $short -like "*$wordToComplete*") {
                $completions += [CompletionResult]::new($short, $short, 'ParameterName', $description)
            }
            if ($long -and $long -like "*$wordToComplete*") {
                $completions += [CompletionResult]::new($long, $long, 'ParameterName', $description)
            }
        }
    }

    $completions
}

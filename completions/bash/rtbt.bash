_rtbt() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    # Built-in palette names
    local palettes="nord dracula gruvbox monokai solarized catppuccin tokyo oceanic palenight onedark vim gotham challenger molokai sonokai serenade vaporwave"

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="rtbt"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        rtbt)
            opts="-i -o -p -c -b -q -h -V --img --out --palette --colors --blur --quiet --no-avg --pixels-area --benchmark --list-palettes --create-palette --export-palette --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --img|-i)
                    # Complete image files
                    COMPREPLY=($(compgen -f -X "!*.@(jpg|jpeg|png|JPG|JPEG|PNG)" -- "${cur}"))
                    [[ ${#COMPREPLY[@]} -eq 0 ]] && COMPREPLY=($(compgen -f -- "${cur}"))
                    return 0
                    ;;
                --out|-o)
                    # Complete with .png extension suggestion
                    if [[ ${cur} != *.* ]]; then
                        COMPREPLY=($(compgen -W "${cur}.png" -- "${cur}"))
                    fi
                    COMPREPLY+=($(compgen -f -- "${cur}"))
                    return 0
                    ;;
                --palette|-p)
                    # Complete with built-in palette names and TOML files
                    COMPREPLY=($(compgen -W "${palettes}" -- "${cur}"))
                    COMPREPLY+=($(compgen -f -X "!*.toml" -- "${cur}"))
                    return 0
                    ;;
                --colors|-c)
                    # Provide common color examples
                    local color_examples="red green blue yellow orange purple white black"
                    COMPREPLY=($(compgen -W "${color_examples}" -- "${cur}"))
                    return 0
                    ;;
                --pixels-area)
                    # Provide common pixel area examples
                    local area_examples="1,1 2,2 3,3 4,4 5,5 2,3 3,2"
                    COMPREPLY=($(compgen -W "${area_examples}" -- "${cur}"))
                    return 0
                    ;;
                --create-palette)
                    # Suggest .toml extension
                    if [[ ${cur} != *.* ]]; then
                        COMPREPLY=($(compgen -W "${cur}.toml" -- "${cur}"))
                    fi
                    COMPREPLY+=($(compgen -f -- "${cur}"))
                    return 0
                    ;;
                --export-palette)
                    # Check if we're completing the first or second argument
                    local export_args=0
                    for (( i=1; i<COMP_CWORD; i++ )); do
                        if [[ "${COMP_WORDS[$i]}" == "--export-palette" ]]; then
                            export_args=$((COMP_CWORD - i - 1))
                            break
                        fi
                    done
                    
                    if [[ $export_args -eq 1 ]]; then
                        # First argument: palette name
                        COMPREPLY=($(compgen -W "${palettes}" -- "${cur}"))
                    elif [[ $export_args -eq 2 ]]; then
                        # Second argument: output file
                        if [[ ${cur} != *.* ]]; then
                            COMPREPLY=($(compgen -W "${cur}.toml" -- "${cur}"))
                        fi
                        COMPREPLY+=($(compgen -f -- "${cur}"))
                    fi
                    return 0
                    ;;
                *)
                    # Default file completion
                    COMPREPLY=($(compgen -f -- "${cur}"))
                    ;;
            esac
            ;;
    esac
}

# Enable completion
if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _rtbt -o nosort -o bashdefault -o default rtbt
else
    complete -F _rtbt -o bashdefault -o default rtbt
fi

# Bash completion for pss CLI

_pss_completions() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    opts="setup decode verify capacity extract info combo-demo tier-setup tier-decode tier-demo help -h --help"

    case "$prev" in
        --corpus|--output|--pool|--file)
            COMPREPLY=( $(compgen -f -- "$cur") $(compgen -W "-" -- "$cur") )
            return 0
            ;;
        setup)
            COMPREPLY=( $(compgen -W "--corpus --output --k --n --min-size --file" -- "$cur") )
            return 0
            ;;
        decode)
            COMPREPLY=( $(compgen -W "--pool --mode --seed-k --path-k --path-len --seed-len --payload --payload-len --k" -- "$cur") )
            return 0
            ;;
        combo-demo)
            COMPREPLY=( $(compgen -W "--k --n" -- "$cur") )
            return 0
            ;;
        verify|capacity|extract|info)
            COMPREPLY=( $(compgen -W "--pool --k --n --max-bytes --select-top" -- "$cur") )
            return 0
            ;;
        tier-setup|tier-decode|tier-demo)
            COMPREPLY=( $(compgen -W "--secret" -- "$cur") )
            return 0
            ;;
        *)
            ;;
    esac

    COMPREPLY=( $(compgen -W "$opts" -- "$cur") )
    return 0
}

complete -F _pss_completions pss

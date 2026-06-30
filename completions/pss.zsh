#compdef pss

_pss() {
    local -a commands
    commands=(
        'setup:select n untouched carriers from corpus'
        'decode:reconstruct seed from k indices (sum or combo mode)'
        'combo-demo:synthetic combo-path roundtrip'
        'verify:k+1 Shamir consistency check'
        'capacity:report L_{n-k+1} max secret size'
        'extract:deterministic transposed read'
        'info:print carrier metadata'
        'tier-setup:fractal encode demo'
        'tier-decode:fractal decode demo'
        'tier-demo:fractal demo'
        'help:show help'
    )

    _arguments -C \
        '1: :->command' \
        '*:: :->args'

    case $state in
        command)
            _describe 'command' commands
            ;;
        args)
            case $words[1] in
                setup)
                    _arguments \
                        '--corpus[corpus directory]:directory:_files -/' \
                        '--output[output directory]:directory:_files -/' \
                        '--k[threshold]:number:' \
                        '--n[share count]:number:' \
                        '--min-size[minimum bytes]:number:' \
                        '--file[secret file]:file:_files'
                    ;;
                decode)
                    _arguments \
                        '--pool[carrier pool]:directory:_files -/' \
                        '--mode[decode mode]:mode:(sum combo)' \
                        '--seed-k[seed threshold]:number:' \
                        '--path-k[path threshold]:number:' \
                        '--path-len[path bytes]:number:' \
                        '--seed-len[seed bytes]:number:' \
                        '--payload[OTP ciphertext]:file:_files' \
                        '--payload-len[plaintext bytes]:number:'
                    ;;
                verify|extract|info|capacity)
                    _arguments \
                        '--pool[carrier pool]:directory:_files -/' \
                        '--k[threshold]:number:' \
                        '--n[share count]:number:' \
                        '--select-top[candidate pool size]:number:' \
                        '--max-bytes[extract limit]:number:'
                    ;;
                tier-setup|tier-decode|tier-demo)
                    _arguments '--secret[byte value]:number:'
                    ;;
            esac
            ;;
    esac
}

_pss "$@"

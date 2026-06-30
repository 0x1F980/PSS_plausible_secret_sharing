# Fish completion for pss CLI

complete -c pss -f

complete -c pss -n "not __fish_seen_subcommand_from setup decode verify capacity extract info combo-demo tier-setup tier-decode tier-demo help" -a "setup decode verify capacity extract info combo-demo tier-setup tier-decode tier-demo help"

complete -c pss -n "__fish_seen_subcommand_from setup" -l corpus -r -d "Corpus directory"
complete -c pss -n "__fish_seen_subcommand_from setup" -l output -r -d "Output directory"
complete -c pss -n "__fish_seen_subcommand_from setup" -l k -r -d "Threshold k"
complete -c pss -n "__fish_seen_subcommand_from setup" -l n -r -d "Share count n"
complete -c pss -n "__fish_seen_subcommand_from setup" -l min-size -r -d "Minimum file size"
complete -c pss -n "__fish_seen_subcommand_from setup" -l file -r -d "Secret file"

complete -c pss -n "__fish_seen_subcommand_from decode" -l pool -r -d "Carrier pool"
complete -c pss -n "__fish_seen_subcommand_from decode" -l mode -r -a "sum combo" -d "Decode mode"
complete -c pss -n "__fish_seen_subcommand_from decode" -l seed-k -r -d "Seed threshold k"
complete -c pss -n "__fish_seen_subcommand_from decode" -l path-k -r -d "Path threshold k"
complete -c pss -n "__fish_seen_subcommand_from decode" -l path-len -r -d "Path byte length"
complete -c pss -n "__fish_seen_subcommand_from decode" -l seed-len -r -d "Seed byte length"
complete -c pss -n "__fish_seen_subcommand_from decode" -l payload -r -d "OTP ciphertext file"
complete -c pss -n "__fish_seen_subcommand_from decode" -l payload-len -r -d "OTP plaintext length"

complete -c pss -n "__fish_seen_subcommand_from verify capacity extract info" -l pool -r -d "Carrier pool"
complete -c pss -n "__fish_seen_subcommand_from capacity" -l k -r
complete -c pss -n "__fish_seen_subcommand_from capacity" -l n -r
complete -c pss -n "__fish_seen_subcommand_from capacity" -l select-top -r
complete -c pss -n "__fish_seen_subcommand_from extract" -l max-bytes -r

complete -c pss -n "__fish_seen_subcommand_from tier-setup tier-decode tier-demo" -l secret -r

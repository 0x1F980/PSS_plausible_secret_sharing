# PowerShell completion for pss CLI

Register-ArgumentCompleter -Native -CommandName pss -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)
    $commands = @(
        'setup', 'decode', 'combo-demo', 'verify', 'capacity', 'extract', 'info',
        'tier-setup', 'tier-decode', 'tier-demo', 'help'
    )
    $flags = @(
        '--corpus', '--output', '--pool', '--k', '--n', '--min-size', '--file',
        '--mode', '--seed-k', '--path-k', '--path-len', '--seed-len', '--payload', '--payload-len',
        '--max-bytes', '--secret', '--select-top'
    )
    $commands + $flags | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
        [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterName', $_)
    }
}

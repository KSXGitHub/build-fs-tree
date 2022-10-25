
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'build-fs-tree' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'build-fs-tree'
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

    $completions = @(switch ($command) {
        'build-fs-tree' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information (use `--help` for more detail)')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information (use `--help` for more detail)')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('create', 'create', [CompletionResultType]::ParameterValue, 'Read YAML from stdin and create a new filesystem tree')
            [CompletionResult]::new('populate', 'populate', [CompletionResultType]::ParameterValue, 'Read YAML from stdin and populate an existing filesystem tree')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'build-fs-tree;create' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information (use `--help` for more detail)')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information (use `--help` for more detail)')
            break
        }
        'build-fs-tree;populate' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information (use `--help` for more detail)')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information (use `--help` for more detail)')
            break
        }
        'build-fs-tree;help' {
            [CompletionResult]::new('create', 'create', [CompletionResultType]::ParameterValue, 'Read YAML from stdin and create a new filesystem tree')
            [CompletionResult]::new('populate', 'populate', [CompletionResultType]::ParameterValue, 'Read YAML from stdin and populate an existing filesystem tree')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'build-fs-tree;help;create' {
            break
        }
        'build-fs-tree;help;populate' {
            break
        }
        'build-fs-tree;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}

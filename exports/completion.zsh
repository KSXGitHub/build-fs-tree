#compdef build-fs-tree

autoload -U is-at-least

_build-fs-tree() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
'-V[Print version]' \
'--version[Print version]' \
":: :_build-fs-tree_commands" \
"*::: :->build-fs-tree" \
&& ret=0
    case $state in
    (build-fs-tree)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:build-fs-tree-command-$line[1]:"
        case $line[1] in
            (create)
_arguments "${_arguments_options[@]}" \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
':TARGET:_files' \
&& ret=0
;;
(populate)
_arguments "${_arguments_options[@]}" \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
':TARGET:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
":: :_build-fs-tree__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:build-fs-tree-help-command-$line[1]:"
        case $line[1] in
            (create)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(populate)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_build-fs-tree_commands] )) ||
_build-fs-tree_commands() {
    local commands; commands=(
'create:Read YAML from stdin and create a new filesystem tree' \
'populate:Read YAML from stdin and populate an existing filesystem tree' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'build-fs-tree commands' commands "$@"
}
(( $+functions[_build-fs-tree__create_commands] )) ||
_build-fs-tree__create_commands() {
    local commands; commands=()
    _describe -t commands 'build-fs-tree create commands' commands "$@"
}
(( $+functions[_build-fs-tree__help__create_commands] )) ||
_build-fs-tree__help__create_commands() {
    local commands; commands=()
    _describe -t commands 'build-fs-tree help create commands' commands "$@"
}
(( $+functions[_build-fs-tree__help_commands] )) ||
_build-fs-tree__help_commands() {
    local commands; commands=(
'create:Read YAML from stdin and create a new filesystem tree' \
'populate:Read YAML from stdin and populate an existing filesystem tree' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'build-fs-tree help commands' commands "$@"
}
(( $+functions[_build-fs-tree__help__help_commands] )) ||
_build-fs-tree__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'build-fs-tree help help commands' commands "$@"
}
(( $+functions[_build-fs-tree__help__populate_commands] )) ||
_build-fs-tree__help__populate_commands() {
    local commands; commands=()
    _describe -t commands 'build-fs-tree help populate commands' commands "$@"
}
(( $+functions[_build-fs-tree__populate_commands] )) ||
_build-fs-tree__populate_commands() {
    local commands; commands=()
    _describe -t commands 'build-fs-tree populate commands' commands "$@"
}

_build-fs-tree "$@"

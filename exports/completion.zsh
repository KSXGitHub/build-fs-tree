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
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
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
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':TARGET:_files' \
&& ret=0
;;
(populate)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':TARGET:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
}

(( $+functions[_build-fs-tree_commands] )) ||
_build-fs-tree_commands() {
    local commands; commands=(
        "create:Read YAML from stdin and create a new filesystem tree at <TARGET>. Merged paths are not allowed" \
"populate:Read YAML from stdin and populate an existing filesystem tree at <TARGET>. Parent directories would be created if they are not already exist" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'build-fs-tree commands' commands "$@"
}
(( $+functions[_build-fs-tree__create_commands] )) ||
_build-fs-tree__create_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'build-fs-tree create commands' commands "$@"
}
(( $+functions[_build-fs-tree__help_commands] )) ||
_build-fs-tree__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'build-fs-tree help commands' commands "$@"
}
(( $+functions[_build-fs-tree__populate_commands] )) ||
_build-fs-tree__populate_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'build-fs-tree populate commands' commands "$@"
}

_build-fs-tree "$@"
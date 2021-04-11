
edit:completion:arg-completer[build-fs-tree] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'build-fs-tree'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'build-fs-tree'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand create 'Read YAML from stdin and create a new filesystem tree at <TARGET>. Merged paths are not allowed'
            cand populate 'Read YAML from stdin and populate an existing filesystem tree at <TARGET>. Parent directories would be created if they are not already exist'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'build-fs-tree;create'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'build-fs-tree;populate'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'build-fs-tree;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}

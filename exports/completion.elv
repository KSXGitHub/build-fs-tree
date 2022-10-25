
use builtin;
use str;

set edit:completion:arg-completer[build-fs-tree] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'build-fs-tree'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'build-fs-tree'= {
            cand -h 'Print help information (use `--help` for more detail)'
            cand --help 'Print help information (use `--help` for more detail)'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand create 'Read YAML from stdin and create a new filesystem tree'
            cand populate 'Read YAML from stdin and populate an existing filesystem tree'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'build-fs-tree;create'= {
            cand -h 'Print help information (use `--help` for more detail)'
            cand --help 'Print help information (use `--help` for more detail)'
        }
        &'build-fs-tree;populate'= {
            cand -h 'Print help information (use `--help` for more detail)'
            cand --help 'Print help information (use `--help` for more detail)'
        }
        &'build-fs-tree;help'= {
            cand create 'Read YAML from stdin and create a new filesystem tree'
            cand populate 'Read YAML from stdin and populate an existing filesystem tree'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'build-fs-tree;help;create'= {
        }
        &'build-fs-tree;help;populate'= {
        }
        &'build-fs-tree;help;help'= {
        }
    ]
    $completions[$command]
}

# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_build_fs_tree_global_optspecs
	string join \n h/help V/version
end

function __fish_build_fs_tree_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_build_fs_tree_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_build_fs_tree_using_subcommand
	set -l cmd (__fish_build_fs_tree_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c build-fs-tree -n "__fish_build_fs_tree_needs_command" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c build-fs-tree -n "__fish_build_fs_tree_needs_command" -s V -l version -d 'Print version'
complete -c build-fs-tree -n "__fish_build_fs_tree_needs_command" -f -a "create" -d 'Read YAML from stdin and create a new filesystem tree'
complete -c build-fs-tree -n "__fish_build_fs_tree_needs_command" -f -a "populate" -d 'Read YAML from stdin and populate an existing filesystem tree'
complete -c build-fs-tree -n "__fish_build_fs_tree_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c build-fs-tree -n "__fish_build_fs_tree_using_subcommand create" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c build-fs-tree -n "__fish_build_fs_tree_using_subcommand populate" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c build-fs-tree -n "__fish_build_fs_tree_using_subcommand help; and not __fish_seen_subcommand_from create populate help" -f -a "create" -d 'Read YAML from stdin and create a new filesystem tree'
complete -c build-fs-tree -n "__fish_build_fs_tree_using_subcommand help; and not __fish_seen_subcommand_from create populate help" -f -a "populate" -d 'Read YAML from stdin and populate an existing filesystem tree'
complete -c build-fs-tree -n "__fish_build_fs_tree_using_subcommand help; and not __fish_seen_subcommand_from create populate help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'

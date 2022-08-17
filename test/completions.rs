#![cfg(feature = "cli")]
use build_fs_tree::program::{
    clap_complete::Shell, clap_utilities::CommandFactoryExtra, main::Args,
};

macro_rules! check {
    ($name:ident: $shell:ident -> $path:literal) => {
        #[test]
        fn $name() {
            eprintln!(
                "check!({name}: {shell} -> {path});",
                name = stringify!($name),
                shell = stringify!($shell),
                path = $path,
            );
            let received = Args::get_completion_string("build-fs-tree", Shell::$shell)
                .expect("get completion string");
            let expected = include_str!($path);
            let panic_message = concat!(
                stringify!($variant),
                " completion is outdated. Re-run generate-completions.sh to update",
            );
            assert!(received == expected, "{panic_message}");
        }
    };
}

check!(bash: Bash -> "../exports/completion.bash");
check!(fish: Fish -> "../exports/completion.fish");
check!(zsh: Zsh -> "../exports/completion.zsh");
check!(powershell: PowerShell -> "../exports/completion.ps1");
check!(elvish: Elvish -> "../exports/completion.elv");

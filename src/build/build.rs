extern crate built;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use std::os::unix::fs::PermissionsExt;

fn main() {
    // Setting up git hooks in the project: rustfmt and so on.
    let git_hooks = format!(
        "git config core.hooksPath {}",
        PathBuf::from("./.hooks").to_str().unwrap()
    );

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", &git_hooks])
            .output()
            .expect("failed to execute git config for hooks");
    } else {
        Command::new("sh")
            .args(&["-c", &git_hooks])
            .output()
            .expect("failed to execute git config for hooks");

        let mut permissions = PathBuf::from("./.hooks/pre-commit")
            .metadata()
            .expect("metadata call failed")
            .permissions();

        if permissions.mode() & 0o111 != 0 {
            permissions.set_mode(0o770);
            fs::set_permissions("./.hooks/pre-commit", permissions)
                .expect("failed to set executable permissions");
        }
    }

    // build and versioning information
    let mut opts = built::Options::default();
    opts.set_dependencies(true);
    // don't fail the build if something's missing, may just be cargo release
    let _ = built::write_built_file_with_opts(
        &opts,
        env!("CARGO_MANIFEST_DIR"),
        format!("{}{}", env::var("OUT_DIR").unwrap(), "/built.rs"),
    );
}

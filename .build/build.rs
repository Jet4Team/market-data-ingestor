extern crate built;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use std::os::unix::fs::PermissionsExt;

fn main() {

    let hooks_dir = if PathBuf::from("../.hooks").exists() {
        PathBuf::from("../.hooks")
    } else  {
        PathBuf::from(".hooks")
    };

    let pre_commit = PathBuf::from(format!("{}/{}", hooks_dir.to_str().unwrap(), "pre-commit"));

    // Setting up git hooks in the project: rustfmt and so on.
    let git_hooks = format!(
        "git config core.hooksPath {}",
        hooks_dir.to_str().unwrap()
    );

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", &git_hooks])
            .output()
            .expect("build: failed to execute git config for hooks");
    } else {
        Command::new("sh")
            .args(&["-c", &git_hooks])
            .output()
            .expect("build: failed to execute git config for hooks");

        let mut permissions = pre_commit
            .metadata()
            .expect("build: failed to get metadata of pre-commit")
            .permissions();

        if permissions.mode() & 0o111 != 0 {
            permissions.set_mode(0o770);
            fs::set_permissions(pre_commit, permissions)
                .expect("build: failed to set executable permissions");
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

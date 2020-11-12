use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(&["describe", "--tags"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}

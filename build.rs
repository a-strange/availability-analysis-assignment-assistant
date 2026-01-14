use std::process::Command;

fn main() {
    // Priority 1: Check for VERSION environment variable (CI)
    if let Ok(version) = std::env::var("VERSION") {
        println!("cargo:rustc-env=APP_VERSION={}", version);
        return;
    }

    // Priority 2: Try git describe (local dev with git)
    if let Ok(output) = Command::new("git")
        .args(&["describe", "--tags", "--always", "--dirty=-modified"])
        .output()
    {
        if output.status.success() {
            let git_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !git_version.is_empty() {
                println!("cargo:rustc-env=APP_VERSION={}", git_version);
                return;
            }
        }
    }

    // Priority 3: Try git commit hash if no tags exist
    if let Ok(output) = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
    {
        if output.status.success() {
            let git_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !git_hash.is_empty() {
                let cargo_version = env!("CARGO_PKG_VERSION");
                println!("cargo:rustc-env=APP_VERSION={}-dev-{}", cargo_version, git_hash);
                return;
            }
        }
    }

    // Priority 4: Fallback to Cargo.toml version + dev suffix
    let cargo_version = env!("CARGO_PKG_VERSION");
    println!("cargo:rustc-env=APP_VERSION={}-dev", cargo_version);

    // Rerun if git HEAD changes (for local development)
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs");
}

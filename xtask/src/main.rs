use std::process::{Command, ExitCode};

type Result<T = ()> = anyhow::Result<T>;

fn main() -> ExitCode {
    let task = std::env::args().nth(1);
    let result = match task.as_deref() {
        Some("ci") => ci(),
        Some("fmt") => fmt(false),
        Some("fmt-check") => fmt(true),
        Some("clippy") => clippy(),
        Some("test") => test(),
        Some("rail-ci") => rail_ci(),
        Some("rail-release") => rail_release(),
        Some(t) => {
            eprintln!("unknown task: {t}");
            eprintln!("available: ci, fmt, fmt-check, clippy, test, rail-ci, rail-release");
            return ExitCode::FAILURE;
        }
        None => {
            eprintln!("usage: cargo xtask <task>");
            eprintln!("available: ci, fmt, fmt-check, clippy, test, rail-ci, rail-release");
            return ExitCode::FAILURE;
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn ci() -> Result {
    rail_unify_check()?;
    fmt(true)?;
    clippy()?;
    test()
}

/// Run cargo rail unify --check (fails if workspace deps are out of sync).
fn rail_unify_check() -> Result {
    let mut cmd = Command::new("cargo");
    cmd.args(["rail", "unify", "--check"]);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be inside workspace");
    cmd.current_dir(root);
    run(cmd)
}

/// Run the full rail CI surface (build + test, all crates).
fn rail_ci() -> Result {
    let mut cmd = Command::new("cargo");
    cmd.args(["rail", "run", "--all", "--surface", "build", "--surface", "test"]);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be inside workspace");
    cmd.current_dir(root);
    run(cmd)
}

/// Publish to crates.io via cargo rail release.
fn rail_release() -> Result {
    let mut cmd = Command::new("cargo");
    cmd.args(["rail", "release"]);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be inside workspace");
    cmd.current_dir(root);
    run(cmd)
}

fn fmt(check: bool) -> Result {
    let mut cmd = cargo();
    cmd.args(["fmt", "--all"]);
    if check {
        cmd.args(["--", "--check"]);
    }
    run(cmd)
}

fn clippy() -> Result {
    let mut cmd = cargo();
    cmd.args([
        "clippy",
        "--workspace",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ]);
    run(cmd)
}

fn test() -> Result {
    // Prefer nextest; fall back to cargo test.
    if which("cargo-nextest") {
        let mut cmd = cargo();
        cmd.args(["nextest", "run", "--workspace"]);
        run(cmd)
    } else {
        let mut cmd = cargo();
        cmd.args(["test", "--workspace"]);
        run(cmd)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn cargo() -> Command {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let mut cmd = Command::new(cargo);
    // Always run from workspace root (two levels up from xtask/).
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be inside workspace");
    cmd.current_dir(root);
    cmd
}

fn run(mut cmd: Command) -> Result {
    let status = cmd.status()?;
    anyhow::ensure!(status.success(), "command failed with {:?}", status.code());
    Ok(())
}

fn which(bin: &str) -> bool {
    Command::new("which")
        .arg(bin)
        .output()
        .is_ok_and(|o| o.status.success())
}

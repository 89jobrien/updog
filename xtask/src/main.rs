use std::io::Write;
use std::process::{Command, ExitCode, Stdio};

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
        Some("sarif") => sarif(),
        Some(t) => {
            eprintln!("unknown task: {t}");
            eprintln!("available: ci, fmt, fmt-check, clippy, test, rail-ci, rail-release, sarif");
            return ExitCode::FAILURE;
        }
        None => {
            eprintln!("usage: cargo xtask <task>");
            eprintln!("available: ci, fmt, fmt-check, clippy, test, rail-ci, rail-release, sarif");
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
    cmd.args([
        "rail",
        "run",
        "--all",
        "--surface",
        "build",
        "--surface",
        "test",
    ]);
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

/// Generate SARIF reports from clippy and cargo rail, written to target/.
///
/// Outputs:
///   target/clippy.sarif   — clippy findings (via clippy-sarif)
///   target/rail-unify.sarif — rail unify findings (converted from rail JSON)
fn sarif() -> Result {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be inside workspace");

    std::fs::create_dir_all(root.join("target"))?;

    // --- clippy SARIF -------------------------------------------------------
    // cargo clippy --message-format=json | clippy-sarif > target/clippy.sarif
    if !which("clippy-sarif") {
        anyhow::bail!("clippy-sarif not found — install with: cargo install clippy-sarif");
    }
    let cargo_bin = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let clippy_proc = Command::new(&cargo_bin)
        .args([
            "clippy",
            "--workspace",
            "--all-targets",
            "--message-format=json",
        ])
        .current_dir(root)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let sarif_out = Command::new("clippy-sarif")
        .stdin(clippy_proc.stdout.expect("clippy stdout"))
        .current_dir(root)
        .output()?;

    // clippy-sarif exits non-zero when there are findings — that's expected
    let clippy_sarif_path = root.join("target/clippy.sarif");
    std::fs::write(&clippy_sarif_path, &sarif_out.stdout)?;
    eprintln!("clippy sarif → {}", clippy_sarif_path.display());

    // --- rail unify SARIF ---------------------------------------------------
    let rail_out = Command::new("cargo")
        .args(["rail", "unify", "--check", "-f", "json"])
        .current_dir(root)
        .output()?;

    // rail exits non-zero when changes are needed — capture output regardless
    let rail_json: serde_json::Value =
        serde_json::from_slice(&rail_out.stdout).unwrap_or(serde_json::json!({"issues": []}));

    let issues = rail_json["issues"].as_array().cloned().unwrap_or_default();
    let results: Vec<serde_json::Value> = issues
        .iter()
        .map(|issue| {
            let msg = issue["message"]
                .as_str()
                .or_else(|| issue["msg"].as_str())
                .unwrap_or("rail finding");
            let code = issue["code"]
                .as_str()
                .or_else(|| issue["rule"].as_str())
                .unwrap_or("cargo-rail/unknown");
            let level = match issue["severity"].as_str().unwrap_or("warning") {
                "error" => "error",
                _ => "warning",
            };
            serde_json::json!({
                "ruleId": code,
                "level": level,
                "message": { "text": msg },
                "locations": []
            })
        })
        .collect();

    let sarif = serde_json::json!({
        "$schema": "https://schemastore.azurewebsites.net/schemas/json/sarif-2.1.0-rtm.5.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "cargo-rail",
                    "informationUri": "https://github.com/loadingalias/cargo-rail",
                    "rules": []
                }
            },
            "results": results
        }]
    });

    let rail_sarif_path = root.join("target/rail-unify.sarif");
    let mut f = std::fs::File::create(&rail_sarif_path)?;
    write!(f, "{}", serde_json::to_string_pretty(&sarif)?)?;
    eprintln!("rail   sarif → {}", rail_sarif_path.display());

    Ok(())
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

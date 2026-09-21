//! Terminal rendering helpers for phase progress and values.

use std::path::Path;

use console::style;

/// Prints a phase heading, marking optional phases explicitly.
pub fn phase_header(id: u8, name: &str, optional: bool) {
    if optional {
        println!(
            "\n{} {}",
            style(format!("Phase {id} (optional):")).bold().dim(),
            style(name).bold()
        );
    } else {
        println!("\n{}", style(format!("Phase {id}: {name}")).bold().cyan());
    }
}

/// Prints a success message with a green check mark.
pub fn success(msg: impl AsRef<str>) {
    println!("{} {}", style("✓").green().bold(), msg.as_ref());
}

/// Prints an indented informational message.
pub fn info(msg: impl AsRef<str>) {
    println!("  {}", msg.as_ref());
}

/// Prints a warning message to standard error.
pub fn warn(msg: impl AsRef<str>) {
    eprintln!("{} {}", style("!").yellow().bold(), msg.as_ref());
}

/// Prints an action prefixed with a dry-run label.
pub fn dry_run(msg: impl AsRef<str>) {
    println!("{} {}", style("[dry-run]").yellow().bold(), msg.as_ref());
}

/// Formats a filesystem path with dim terminal styling.
pub fn path_str(p: &Path) -> String {
    style(p.display()).dim().to_string()
}

/// Formats a command or code fragment with cyan terminal styling.
pub fn code(s: impl AsRef<str>) -> String {
    style(s.as_ref()).cyan().to_string()
}

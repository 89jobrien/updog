use std::path::Path;

use console::style;

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

pub fn success(msg: impl AsRef<str>) {
    println!("{} {}", style("✓").green().bold(), msg.as_ref());
}

pub fn info(msg: impl AsRef<str>) {
    println!("  {}", msg.as_ref());
}

pub fn warn(msg: impl AsRef<str>) {
    eprintln!("{} {}", style("!").yellow().bold(), msg.as_ref());
}

pub fn dry_run(msg: impl AsRef<str>) {
    println!("{} {}", style("[dry-run]").yellow().bold(), msg.as_ref());
}

pub fn path_str(p: &Path) -> String {
    style(p.display()).dim().to_string()
}

pub fn code(s: impl AsRef<str>) -> String {
    style(s.as_ref()).cyan().to_string()
}

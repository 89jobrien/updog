use clap::{Parser, Subcommand};
use miette::Result;
use tracing_subscriber::{EnvFilter, fmt};

mod adapters;
mod phase;
mod phases;
mod run;
mod ui;

#[derive(Parser)]
#[command(
    name = "ail",
    about = "Agent Improvement Loop — systematic agent behavior improvement from observed traces"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the improvement loop (all phases or from a specific phase)
    Run {
        /// Which agent's traces to analyze
        #[arg(long, default_value = "current")]
        agent: String,

        /// Days of trace history to scan
        #[arg(long, default_value_t = 30)]
        since: u32,

        /// Start from this phase (1–7)
        #[arg(long, default_value_t = 1)]
        phase: u8,

        /// Print what would be done without writing files or running commands
        #[arg(long)]
        dry_run: bool,
    },
}

fn main() -> Result<()> {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn")),
        )
        .with_target(false)
        .without_time()
        .init();

    let cli = Cli::parse();
    match cli.command {
        Command::Run {
            agent,
            since,
            phase,
            dry_run,
        } => {
            let source = Box::new(adapters::coursers::CourserTraceSource);
            let config = run::RunConfig::new(agent, since, phase, dry_run, source)
                .map_err(|e| miette::miette!("{e:#}"))?;
            run::execute(config).map_err(|e| miette::miette!("{e:#}"))?;
        }
    }
    Ok(())
}

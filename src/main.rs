mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cli_app")]
#[command(about = "A sample CLI app in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Load files from repository
    #[command(alias = "l")]
    Load {
        #[arg(short, long)]
        verbose: bool,
    },

    /// Save data to repository
    #[command(alias = "s")]
    Save {
        #[arg(short, long)]
        verbose: bool,
    },

    /// Query functions by pattern
    #[command(alias = "q")]
    Query {
        pattern: Option<String>,
        #[arg(short, long)]
        verbose: bool,
    },

    /// Clear the database or functions by pattern
    #[command(alias = "c")]
    Clear {
        patterns: Vec<String>,
        #[arg(short, long)]
        verbose: bool,
    },

    /// Invoke a function by name
    #[command(alias = "i")]
    Invoke {
        function: String,
        params: Vec<String>,
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Load { verbose }                     => commands::load::run(verbose),
        Commands::Save { verbose }                     => commands::save::run(verbose),
        Commands::Query { pattern, verbose }           => commands::query::run(pattern, verbose),
        Commands::Clear { patterns, verbose }          => commands::clear::run(patterns, verbose),
        Commands::Invoke { function, params, verbose } => commands::invoke::run(function, params, verbose),
    }
}

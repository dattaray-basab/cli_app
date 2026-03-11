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
    /// Load and compile files from repository
    #[command(alias = "l")]
    Load {
        #[arg(short, long)]
        verbose: bool,
    },

    /// Save data to repository
    #[command(alias = "s")]
    Save {
        /// Optional pattern to filter what is saved
        pattern: Option<String>,
        #[arg(short, long)]
        verbose: bool,
        /// Persist functions after save (skip cleanup)
        #[arg(short, long)]
        keep: bool,
    },

    /// Query/find functions by FQN/pattern (supports wildcards)
    #[command(alias = "q")]
    Query {
        /// Pattern(s) to match (supports wildcards, e.g. "math/*")
        patterns: Vec<String>,
        #[arg(short, long)]
        verbose: bool,
    },

    /// Reset the database, or clear functions by FQN/pattern
    #[command(alias = "c")]
    Clear {
        /// Pattern(s) to match (supports wildcards)
        patterns: Vec<String>,
        #[arg(short, long)]
        verbose: bool,
    },

    /// Invoke a specific function by name or FQN
    #[command(alias = "i")]
    Invoke {
        /// Function name or FQN
        function: String,
        /// Parameters to pass to the function
        params: Vec<String>,
        #[arg(short, long)]
        verbose: bool,
    },

    /// Run built-in integration test
    #[command(alias = "t")]
    Test {
        #[arg(short, long)]
        verbose: bool,
        /// Persist functions after test (skip cleanup)
        #[arg(short, long)]
        keep: bool,
    },

    /// Generate DSL functions given a kodegen_config_key
    #[command(alias = "g")]
    Generate {
        /// Config key (function FQN or directory)
        kodegen_config_key: String,
        #[arg(short, long)]
        verbose: bool,
        /// Persist functions after generation (skip cleanup)
        #[arg(short, long)]
        keep: bool,
        /// Provision workspace after generation
        #[arg(short, long)]
        provision: bool,
    },

    /// Interactive step-by-step debugging of expression evaluation
    #[command(alias = "w")]
    Watch {
        /// Expression to evaluate
        expression: String,
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Load { verbose } =>
            commands::load::run(verbose),
        Commands::Save { pattern, verbose, keep } =>
            commands::save::run(pattern, verbose, keep),
        Commands::Query { patterns, verbose } =>
            commands::query::run(patterns, verbose),
        Commands::Clear { patterns, verbose } =>
            commands::clear::run(patterns, verbose),
        Commands::Invoke { function, params, verbose } =>
            commands::invoke::run(function, params, verbose),
        Commands::Test { verbose, keep } =>
            commands::test::run(verbose, keep),
        Commands::Generate { kodegen_config_key, verbose, keep, provision } =>
            commands::generate::run(kodegen_config_key, verbose, keep, provision),
        Commands::Watch { expression, verbose } =>
            commands::watch::run(expression, verbose),
    }
}

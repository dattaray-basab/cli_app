mod commands;
mod prompt;

use clap::{Parser, Subcommand};

const LONG_ABOUT: &str = "\
A sample CLI app in Rust — demonstrating commands, aliases, flags, and interactive prompts.

COMMANDS (with aliases):
  load,     l   Load and compile files from repository
  save,     s   Save data to repository
  query,    q   Query/find functions by FQN/pattern (supports wildcards)
  clear,    c   Reset the database, or clear functions by FQN/pattern
  invoke,   i   Invoke a specific function by name or FQN
  test,     t   Run built-in integration test
  generate, g   Generate DSL functions given a config key
  watch,    w   Interactive step-by-step debugging of expression evaluation

FLAGS:
  -v, --verbose     Enable verbose output
  -k, --keep        Persist functions after command (skip cleanup)
  -p, --provision   Provision workspace after generation  (generate only)

EXAMPLES:
  cli_app load -v
  cli_app l -v
  cli_app save
  cli_app save mypattern -k
  cli_app query \"math/*\" -v
  cli_app q \"*Add*\" -v
  cli_app clear -v
  cli_app clear math/foo math/bar -v
  cli_app c \"*Add2*\" -v
  cli_app invoke MyFunc 1 2 3 -v
  cli_app i MyFunc 1 2 3
  cli_app test -v
  cli_app test -v -k
  cli_app generate default -v
  cli_app g mykey -v -k -p
  cli_app watch \"sum(n1, sum(n2, n3))\" -v
  cli_app w \"sum(n1, n2)\"

  # Omit required args to be prompted interactively:
  cli_app generate
  cli_app invoke
  cli_app watch";

#[derive(Parser)]
#[command(name = "cli_app")]
#[command(about = "A sample CLI app in Rust")]
#[command(long_about = LONG_ABOUT)]
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
        /// Function name or FQN (prompted if omitted)
        function: Option<String>,
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
        /// Config key (function FQN or directory) — prompted if omitted
        kodegen_config_key: Option<String>,
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
        /// Expression to evaluate — prompted if omitted
        expression: Option<String>,
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

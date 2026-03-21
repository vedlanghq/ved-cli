use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ved")]
#[command(about = "Ved language CLI - The Deterministic Control Plane Interpreter", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compiles a Ved source file into executable bytecode
    Compile {
        /// The `.ved` source file to compile
        file: PathBuf,
        
        /// Output file name (default: bundle.vedc)
        #[arg(short, long, default_value = "bundle.vedc")]
        output: PathBuf,
    },
    /// Runs a compiled Ved bytecode bundle
    Run {
        /// The `.vedc` target bundle to run
        bundle: PathBuf,
        
        /// Enable verbose diagnostic logging
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Compile { file, output } => {
            println!("Compiling {:?} to {:?}...", file, output);
            // let source = std::fs::read_to_string(file).expect("Failed to read file");
            // ved_compiler::compile_source(&source)...
        }
        Commands::Run { bundle, verbose } => {
            println!("Running bundle at {:?} (verbose: {})", bundle, verbose);
            // ved_runtime::boot(bundle)...
        }
    }
}

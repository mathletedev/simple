pub mod compiler;
pub mod config;
pub mod simulator;
pub mod types;

use clap::{Parser, Subcommand};
use compiler::compiler::Compiler;
use simulator::simpletron::Simpletron;
use std::path::PathBuf;

/// Simple compiler and simulator
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Compile a Simple program to SML
    Com {
        path: PathBuf,

        #[clap(short, long)]
        out: Option<PathBuf>,
    },
    /// Simulate SML with the Simpletron
    Sim { path: PathBuf },
}

// TODO: implement floating-point numbers

fn main() {
    let args = Args::parse();

    match &args.cmd {
        Commands::Com { path, out } => {
            let mut compiler = Compiler::new();

            compiler.compile(
                path.to_path_buf(),
                out.to_owned().unwrap_or(PathBuf::from("./out.sml")),
            );
        }
        Commands::Sim { path } => {
            let mut simpletron = Simpletron::new();

            if simpletron.load(path.to_path_buf()).is_err() {
                return;
            }
            simpletron.simulate();
        }
    }
}

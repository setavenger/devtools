mod convert;
mod reverse;
mod utils;

use clap::{Parser, Subcommand};
use reverse::reverse_byte_slice;
use std::io::{self, BufRead, IsTerminal};

#[derive(Debug, Parser)]
#[command(name = "devtools")]
#[command(about = "A collection of handy tools for developing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Convert {
        data: Option<String>,
        #[arg(short, long)]
        from: convert::StringConversionOptions,
        #[arg(short, long)]
        to: convert::StringConversionOptions,
    },
    Reverse {
        data: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Convert { data, from, to } => {
            match data {
                Some(input) => convert::parse_data(input, from, to),
                None => {
                    let stdin = io::stdin();
                    let handle = stdin.lock();
                    if handle.is_terminal() {
                        return;
                    }
                    for line in handle.lines() {
                        let line = line.expect("Failed to read line");
                        if line.trim().is_empty() {
                            continue; // Skip empty lines
                        }
                        convert::parse_data(line, from, to)
                    }
                }
            }
        }
        Commands::Reverse { data } => {
            match data {
                Some(input) => reverse_byte_slice(&input),
                None => {
                    let stdin = io::stdin();
                    let handle = stdin.lock();
                    if handle.is_terminal() {
                        return;
                    }
                    for line in handle.lines() {
                        let line = line.expect("Failed to read line");
                        if line.trim().is_empty() {
                            continue; // Skip empty lines
                        }
                        reverse_byte_slice(&line)
                    }
                }
            }
        }
    }
}

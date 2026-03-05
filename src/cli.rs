use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "msearch", version, about = "Local full-text search engine (MVP)")]
pub struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Build an index from documents under PATH and save it to DB directory.
    Index {
        /// Root path to scan (directory or file)
        path: String,
        /// DB directory path
        #[arg(long)]
        db: String,
    },
    /// Search with QUERY using existing DB directory.
    Query {
        query: String,
        /// DB directory path
        #[arg(long)]
        db: String,
        /// Number of results
        #[arg(long, default_value_t = 10)]
        top: usize,
    },
}

pub fn run() -> Result<()> {
    let args = Args::parse();
    match args.cmd {
        Command::Index { path, db } => {
            println!("TODO: index path={path} db={db}");
        }
        Command::Query { query, db, top } => {
            println!("TODO: query q={query} db={db} top={top}");
        }
    }
    Ok(())
}

mod cli;
mod types;
mod tokenize;

mod io;
mod index;
mod query;

fn main() -> anyhow::Result<()> {
    cli::run()
}

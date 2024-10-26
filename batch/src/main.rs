use anyhow::Result;

use clap::{Parser, Subcommand};

use batch::subcommand::{persist_absence, persist_existence};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::PersistAbsence(args) => persist_absence::run(args).await,
        Command::PersistExistence(args) => persist_existence::run(args).await,
    }
}

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    PersistExistence(persist_existence::Args),
    PersistAbsence(persist_absence::Args),
}

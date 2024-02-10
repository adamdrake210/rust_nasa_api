use clap::{Parser, Subcommand};
mod cmd;


#[derive(Debug, Parser)]
#[clap(author, about, version)]
#[command(name = "nasa")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Apod(cmd::apod::Cli),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv::dotenv().ok();
  let cli = Cli::parse();

  match cli.command {
    Command::Apod(apod) => apod.run(),
  }
}

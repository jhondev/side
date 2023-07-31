use clap::{Parser, Subcommand};
mod cmd;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// Output in JSON format
    #[clap(global = true, long)]
    json: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Setup(cmd::setup::Args),
}

fn main() {
    let cli = match Cli::try_parse() {
        Ok(args) => args,
        Err(e) => {
            println!("{}", e);
            std::process::exit(0);
        }
    };

    // match Commands::exec(cli).awai
    match cli.command {
        Commands::Setup(args) => cmd::setup::exec(args, cli.json),
    }
}

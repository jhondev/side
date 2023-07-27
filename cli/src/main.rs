use clap::{Parser, Subcommand};

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
    Setup,
}

fn main() {
    let args = match Cli::try_parse() {
        Ok(args) => args,
        Err(e) => {
            println!("{}", e);
            std::process::exit(0);
        }
    };

    // match Commands::exec(cli).awai
    match args.command {
        Commands::Setup => println!("you entered setup"),
    }
}

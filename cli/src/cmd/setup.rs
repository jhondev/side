use clap::Parser;

 #[derive(Parser, Debug)]
 pub struct Args {
   #[arg()]
   name: String
 }

//  pub async fn command(_args: Args, _json: bool) -> Result<()> {
//     println!("Setup command")
//  }

pub fn exec(args: Args, _json: bool) {
   println!("Setup command: name {}", args.name)
}
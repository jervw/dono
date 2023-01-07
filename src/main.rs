use clap::Parser;
use dono::*;
use std::process;

use crate::config::Config;

#[derive(Parser)]
#[clap(
    name = "dono",
    about = "A CLI tool to show your GitHub contributions",
    version = "0.1.0"
)]
struct Args {
    /// GitHub user name
    #[clap(name = "USER_NAME")]
    user_name: String,
}

fn main() {
    let args = Args::parse();

    let config = match Config::new() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    println!("config: {:?}", config);

    let dono = Dono::new(config);
    let contributions = dono.get_contributions(args.user_name);

    if contributions.len() > 0 {
        dono.print_contributions(contributions);
    }
}

use ansi_term::Style;
use clap::Parser;
use dono::*;
use std::process;

mod config;
use config::Config;

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

    match post_query(args.user_name) {
        Ok(response) => {
            let contributions = parse_contributions(response);
            // total contributions in bold style
            println!(
                "\n{} {}\n",
                Style::new().bold().paint("Total contributions:"),
                get_total_contributions(&contributions)
            );

            print_contributions(contributions);
        }
        Err(e) => println!("Error: {e}"),
    }
}

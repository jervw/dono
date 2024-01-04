use clap::Parser;

pub mod config;
pub mod dono;
pub mod utils;

use config::Config;
use dono::*;

#[derive(Parser)]
#[clap(
    name = "dono",
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
)]
struct Args {
    /// GitHub user name
    #[clap(name = "user_name")]
    user_name: String,

    #[arg(short, long, default_value_t = String::from(""))]
    week_start_day: String,
}

fn main() {
    let args = Args::parse();

    let mut config = match Config::new() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };

    if !args.week_start_day.is_empty() {
        config.week_start_day = args.week_start_day;
        let _ = config.rewrite_config_file();
    }

    let dono = Dono::new(config);
    let contributions = dono.get_contributions(args.user_name);

    if !contributions.is_empty() {
        dono.print_contributions(&contributions);
    }
}

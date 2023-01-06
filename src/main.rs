use clap::Parser;
use dono::*;

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

    match post_query(args.user_name) {
        Ok(response) => {
            let contributions = parse_contributions(response);
            println!(
                "Total contributions: {}", get_total_contributions(&contributions)
            );
            print_contributions(contributions);
        }
        Err(e) => println!("Error: {}", e),
    }
}

use std::process;

use dono::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // handle arguments
    if args.len() > 1 {
        let query = &args[1];
        match &query[..] {
            query => (),
        }
    }
}

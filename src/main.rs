use std::env;

use pathifier::{deduplicate_path, get_delimiter};

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: pathifier <path-string>");
        std::process::exit(1);
    }

    let delimiter = get_delimiter();
    let result = deduplicate_path(&args[1], delimiter);
    println!("{}", result);
}

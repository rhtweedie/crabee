use crabee::{load_dict, solve};
use std::env::args;

fn main() {
    // Take input of dictionary, character list and central character
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        println!("Usage:");
        println!("  {} [dictionary file path] [character set]", args[0]);
        return;
    }
    let chars: Vec<char> = args[2].chars().collect();
    let dict = load_dict(&args[1]);
    let results = solve(&chars, chars[0], 4, &dict);
    // Output results
    for result in results {
        println!("{}", result)
    }
}

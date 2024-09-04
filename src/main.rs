use std::env::args;
use std::fs::read_to_string;

fn main() {
    // Take input of dictionary, character list and central character
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        println!("Usage:");
        println!("  {} [dictionary file path] [character set]", args[0]);
        return;
    }
    let chars: Vec<char> = args[2].chars().collect();
    let dict_str = read_to_string(&args[1]).expect("Error reading dictionary.");
    let dict: Vec<&str> = dict_str.lines().collect();
    let results = solve(&chars, chars[0], &dict);
    // Output results
    for result in results {
        println!("{}", result)
    }
}

fn solve(chars: &[char], centre_char: char, dictionary: &[&str]) -> Vec<String> {
    dictionary
        .into_iter()
        .filter(|word| check_chars(chars, centre_char, word))
        .map(|word| word.to_string())
        .collect()
}

/// Returns whether all characters in `word` are contained in `chars`.
fn check_chars(chars: &[char], centre_char: char, word: &str) -> bool {
    word.chars().all(|c| chars.contains(&c)) && word.contains(centre_char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        assert_eq!(
            solve(
                &['f', 'o', 'c', 'e'],
                'f',
                vec!["foo".to_string(), "bar".to_string(), "coffee".to_string()]
            ),
            vec!["foo", "coffee"]
        );
    }
}

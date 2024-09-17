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
    let dict = load_dict(&args[1]);
    let results = solve(&chars, chars[0], 4, &dict);
    // Output results
    for result in results {
        println!("{}", result)
    }
}

fn solve(chars: &[char], centre_char: char, min_len: usize, dictionary: &[String]) -> Vec<String> {
    dictionary
        .into_iter()
        .filter(|word| check_word(chars, centre_char, min_len, word))
        .map(|word| word.to_string())
        .collect()
}

/// Returns whether all characters in `word` are contained in `chars`.
fn check_word(chars: &[char], centre_char: char, min_len: usize, word: &str) -> bool {
    let word = word.to_lowercase();
    word.chars().all(|c| chars.contains(&c)) && word.contains(centre_char) && word.len() >= min_len
}

fn load_dict(path: &str) -> Vec<String> {
    let dict_str = read_to_string(path).expect("Error reading dictionary.");
    dict_str.lines().map(ToOwned::to_owned).collect()
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
                4,
                &[
                    "foo".to_string(),
                    "fooo".to_string(),
                    "bar".to_string(),
                    "coffee".to_string()
                ]
            ),
            vec!["fooo", "coffee"]
        );
    }
}

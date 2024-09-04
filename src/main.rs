fn main() {
    println!("Hello, Rustacean!");
}

fn solve(chars: &[char], centre_char: char, dictionary: Vec<String>) -> Vec<String> {
    dictionary
        .into_iter()
        .filter(|word| check_chars(chars, centre_char, word))
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

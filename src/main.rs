fn main() {
    println!("Hello, Rustacean!");
}

fn solve(chars: &[char], dictionary: Vec<String>) -> Vec<String> {
    dictionary
        .into_iter()
        .filter(|word| check_chars(chars, &word))
        .collect()
}

/// Returns whether all characters in `word` are contained in `chars`.
fn check_chars(chars: &[char], word: &str) -> bool {
    word.chars().all(|c| chars.contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        assert_eq!(
            solve(
                &['f', 'o', 'c', 'e'],
                vec!["foo".to_string(), "bar".to_string(), "coffee".to_string()]
            ),
            vec!["foo", "coffee"]
        );
    }
}

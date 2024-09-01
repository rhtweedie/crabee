fn main() {
    println!("Hello, Rustacean!");
}

fn solve(chars: &[char], dictionary: Vec<String>) -> Vec<String> {
    let mut words = Vec::new();
    for word in dictionary {
        if check_chars(chars, &word) {
            words.push(word)
        }
    }
    words
}

/// Returns whether all characters in `word` are contained in `chars`.
fn check_chars(chars: &[char], word: &str) -> bool {
    for c in word.chars() {
        if !chars.contains(&c) {
            return false;
        }
    }
    true
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

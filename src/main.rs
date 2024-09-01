fn main() {
    println!("Hello, Rustacean!");
}

fn solve(chars: Vec<char>, dictionary: Vec<String>) -> Vec<String> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        assert_eq!(
            solve(
                vec!['f', 'o', 'c'],
                vec!["foo".to_string(), "bar".to_string()]
            ),
            vec!["foo"]
        );
    }
}

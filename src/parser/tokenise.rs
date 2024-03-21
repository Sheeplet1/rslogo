/// Tokenises an Logo script into a vector of tokens. Each token is an instruction
/// or value.
///
/// # Examples
///
/// Consider the Logo script:
/// ```Logo
/// PENDOWN
///
/// SETPENCOLOR "1
/// FORWARD "100
/// ```
///
/// Tokenising this script would result in the following vector:
/// ```rust
/// vec!["PENDOWN", "SETPENCOLOR" "\"1", "FORWARD" "\"100"]
/// ````
pub fn tokenize_script(contents: &str) -> Vec<&str> {
    let tokens: Vec<&str> = contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with("//"))
        .collect();

    tokens
        .iter()
        .flat_map(|line| line.split_whitespace())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_script() {
        let script = r#"
        PENDOWN

        SETPENCOLOR "1
        FORWARD "100
        "#;

        let tokens = tokenize_script(script);
        assert_eq!(
            tokens,
            vec!["PENDOWN", "SETPENCOLOR", "\"1", "FORWARD", "\"100"]
        );
    }
}

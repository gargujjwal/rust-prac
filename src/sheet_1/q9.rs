pub fn filter_long_strings(input: &Vec<String>) -> Vec<String> {
    // this works
    input
        .iter()
        .filter(|s| s.len() > 3)
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    // this is better
    input.iter().filter(|s| s.len() > 3).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_q9() {
        let list = vec![
            "a".to_string(),
            "rust".to_string(),
            "hi".to_string(),
            "code".to_string(),
        ];
        let filtered = filter_long_strings(&list);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0], "rust");
    }
}

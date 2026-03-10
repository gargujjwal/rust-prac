pub fn first_word(s: &str) -> &str {
    // split method returns an iterator, next method chooses first word and
    // unwrao_or method of option is used to give s as default value
    s.split(" ").next().unwrap_or(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_q3() {
        let s = String::from("Rust is fast");
        assert_eq!(first_word(&s), "Rust");
        assert_eq!(first_word("Single"), "Single");
    }
}

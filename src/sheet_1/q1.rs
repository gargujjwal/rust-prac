// Question 1: The "Manual" Move

// In many languages, assigning a = b creates a reference or a shallow copy. In
// Rust, it moves. Write a function
// consume_and_return_length(s: String) -> (String, usize) that takes ownership of
// a String, calculates its length, and then returns both the string and the
// length so the caller can continue using the original string.

pub fn consume_and_return_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_q1() {
        let s1 = String::from("Rustacean");
        let (s2, len) = consume_and_return_length(s1);
        assert_eq!(s2, "Rustacean");
        assert_eq!(len, 9);
    }
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_q10() {
        let s1 = "short";
        let s2 = "much longer";
        assert_eq!(longest(s1, s2), "much longer");
    }
}

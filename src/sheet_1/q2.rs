pub fn modify_and_measure(s: &mut String) -> usize {
    let len = s.len();
    s.push('!'); // Mutable borrow happens here
    len
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_q2() {
        let mut my_str = String::from("Hello");
        let length = modify_and_measure(&mut my_str);
        assert_eq!(my_str, "Hello!");
        assert_eq!(length, 5);
    }
}

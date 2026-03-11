pub fn find_and_replace(target: String, list: &mut Vec<String>, replacement: &str) {
    // solving using indices
    let mut i = 0;
    while i < list.len() {
        if list[i] == target {
            list[i] = replacement.to_string();
            break;
        }

        i += 1;
    }

    // solving using iterator
    for s in list.iter_mut() {
        if *s == target {
            *s = replacement.to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_q5() {
        let mut words = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
        ];
        find_and_replace("banana".to_string(), &mut words, "orange");
        assert_eq!(words[1], "orange");
    }
}

pub fn sum_even_indices(slice: &[i32]) -> i32 {
    let mut i = 0;
    let mut sum = 0;
    while i < slice.len() {
        sum += slice[i];
        i += 2;
    }

    sum
}

pub fn another_way(slice: &[i32]) -> i32 {
    slice.iter().step_by(2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_q7() {
        let arr = [10, 20, 30, 40, 50];
        assert_eq!(sum_even_indices(&arr), 90); // 10 + 30 + 50
    }
}

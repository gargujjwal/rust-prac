/* // WHY DOES THIS FAIL?
pub fn get_dangling_reference() -> &String {
    let s = String::from("I am short lived");
    &s
}

It fails because it is returning a binding which will drop as soon as the function
returns. Thus the reference will become dangling which is not allowed in rust.
There can be no reference to a binding which has been dropped
*/

pub fn get_safe_string() -> String {
    String::from("I am short lived")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_q4() {
        let result = get_safe_string();
        assert_eq!(result, "I am short lived");
    }
}

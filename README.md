# Practice Sheet for Learning Rust

## Practice Sheet 1: Ownership, Borrowing, and Memory Management

### Question 1: The "Manual" Move

In many languages, assigning `a = b` creates a reference or a shallow copy. In Rust, it moves. Write a function `consume_and_return_length(s: String) -> (String, usize)` that takes ownership of a String, calculates its length, and then returns both the string and the length so the caller can continue using the original string.

**File:** `src/sheet_1/q1.rs`

```rust
pub fn consume_and_return_length(s: String) -> (String, usize) {
    // Your code here
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

```

### Question 2: Mutable Borrowing Limits

Refactor the following code to fix the compiler error. You must modify the function so that it successfully appends "!" to a string while also printing its length, but you must respect the rule that **you cannot have a mutable reference and an immutable reference active at the same time.**

**File:** `src/sheet_1/q2.rs`

```rust
// FIX THIS CODE
pub fn modify_and_measure(s: &mut String) -> usize {
    let len = s.len(); // Immutable borrow
    s.push('!');       // Mutable borrow happens here
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

```

### Question 3: The Sliced Word

Write a function `first_word(s: &str) -> &str` that returns a slice representing the first word of a string. If the string contains no spaces, return the entire string. This will test your understanding of **String Slices (`&str`)** and how they act as views into memory.

**File:** `src/sheet_1/q3.rs`

```rust
pub fn first_word(s: &str) -> &str {
    // Your code here
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

```

### Question 4: Scope and Life of a Reference

Explain (in a comment) why the following code fails to compile and then rewrite the function `get_dangling_reference` so that it returns a owned `String` instead of a reference, explaining why this is the idiomatic Rust solution for returning new data.

**File:** `src/sheet_1/q4.rs`

```rust
/* // WHY DOES THIS FAIL?
pub fn get_dangling_reference() -> &String {
    let s = String::from("I am short lived");
    &s
}
*/

pub fn get_safe_string() -> String {
    // Your code here
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

```

### Question 5: Borrowing in Loops

Create a function `find_and_replace(target: String, list: &mut Vec<String>, replacement: &str)` that searches for the `target` string inside a vector. If found, replace that element with the `replacement` string. You will encounter issues if you try to iterate while mutating; solve this by using indexes or specific Iterator methods.

**File:** `src/sheet_1/q5.rs`

```rust
pub fn find_and_replace(target: String, list: &mut Vec<String>, replacement: &str) {
    // Your code here
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_q5() {
        let mut words = vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()];
        find_and_replace("banana".to_string(), &mut words, "orange");
        assert_eq!(words[1], "orange");
    }
}

```

### Question 6: Partial Moves in Patterns

Rust allows "destructuring" structs. However, if you destructure a struct that contains a `String`, you might accidentally move a part of the struct while leaving the rest.
Given the `User` struct below, write a function `get_name_and_keep_user(user: User) -> (String, User)` that returns the name and the original user. Note: You will need to implement/derive `Clone` or `Copy` or manually reconstruct the struct. Choose the most memory-efficient way.

**File:** `src/sheet_1/q6.rs`

```rust
pub struct User {
    pub name: String,
    pub age: u32,
}

pub fn get_name_and_keep_user(user: User) -> (String, User) {
    // Your code here
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_q6() {
        let u = User { name: String::from("Alice"), age: 30 };
        let (name, user_back) = get_name_and_keep_user(u);
        assert_eq!(name, "Alice");
        assert_eq!(user_back.age, 30);
    }
}

```

### Question 7: Array Slices and Fixed Lengths

Write a function `sum_even_indices(slice: &[i32]) -> i32` that takes a slice of integers and returns the sum of all elements located at an even index. This helps you practice working with **Anonymous Slices** which are common in Rust performance tuning.

**File:** `src/sheet_1/q7.rs`

```rust
pub fn sum_even_indices(slice: &[i32]) -> i32 {
    // Your code here
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

```

### Question 8: Ownership in Struct Methods

Implement a `Buffer` struct that holds a `Vec<u8>`. Implement two methods:

1. `add_data(&mut self, data: u8)`: Takes a mutable reference to add data.
2. `clear(self)`: Takes **ownership** of the buffer to clear it and destroy the instance.
   Explain why `clear` taking `self` instead of `&mut self` might be a design choice in Rust.

**File:** `src/sheet_1/q8.rs`

```rust
pub struct Buffer {
    pub data: Vec<u8>,
}

impl Buffer {
    pub fn add_data(&mut self, data: u8) { /* ... */ }
    pub fn clear(self) { /* ... */ }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_q8() {
        let mut buf = Buffer { data: vec![1, 2, 3] };
        buf.add_data(4);
        assert_eq!(buf.data.len(), 4);
        buf.clear();
        // buf.add_data(5); // This should fail to compile if uncommented
    }
}

```

### Question 9: Cloned vs. Referenced Iteration

You have a `Vec<String>`. You want to create a new `Vec<String>` that only contains strings longer than 3 characters. Write a function `filter_long_strings(input: &Vec<String>) -> Vec<String>`.
Inside, you must iterate over the references and decide whether to `.clone()` the strings. Explain the performance implication of cloning vs. moving here.

**File:** `src/sheet_1/q9.rs`

```rust
pub fn filter_long_strings(input: &Vec<String>) -> Vec<String> {
    // Your code here
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_q9() {
        let list = vec!["a".to_string(), "rust".to_string(), "hi".to_string(), "code".to_string()];
        let filtered = filter_long_strings(&list);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0], "rust");
    }
}

```

### Question 10: The Lifetime Puzzle (Introduction)

Rust uses lifetimes to ensure references are valid. Without using the `'a` syntax explicitly (if possible), fix the following code so that it compiles. The function should return the longer of two string slices. If you find you _must_ use lifetime annotations, try to explain why the compiler cannot "elide" (guess) them in this specific case.

**File:** `src/sheet_1/q10.rs`

```rust
// FIX THIS FUNCTION
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
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

```

---

### Setup Instructions

1. In your `src/sheet_1/mod.rs`, add `pub mod q1; pub mod q2; ...` for all ten questions.
2. In your `src/main.rs`, add `mod sheet_1;`.
3. Try to solve them without using `.clone()` unless specifically asked—it makes the "Ownership struggle" more educational!

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

## Practice Sheet 2: Structs, Enums, and Pattern Matching

Now that you've wrestled with the Borrow Checker, let's look at how Rust handles data structures. This sheet is significantly harder.

**Directory Structure:** `src/sheet_2/q1.rs`... (don't forget to update `mod.rs`)

### Question 1: The Result Wrapper

Create a function `divide(a: f64, b: f64) -> Result<f64, String>`. If `b` is 0.0, return an `Err` with a message. In your test, use a `match` statement to assert the value.

- **Goal:** Understand how Rust handles errors without exceptions.

### Question 2: Custom Enum with Data

Define an enum `Shape` that can be `Circle(f64)` or `Rectangle(f64, f64)`. Write a method `area(&self) -> f64` for this enum.

- **Goal:** Learn "Algebraic Data Types" (Enums that hold data).

### Question 3: Pattern Matching and Guards

Write a function `evaluate_age(age: u32) -> &'static str` that uses a `match` block.

- 0-12: "Child"
- 13-19: "Teen"
- 20+: "Adult"
- Add a "match guard" (an `if` inside the match) to return "Golden Year" if the age is exactly 100.

### Question 4: Option Chaining

You have a `struct Profile { nickname: Option<String> }` and a `struct User { profile: Option<Profile> }`. Write a function `get_nickname(user: &User) -> String` that returns the nickname if it exists, otherwise returns "Guest". Use `.and_then()` or `.map()` rather than nested `if let` statements.

### Question 5: Struct Update Syntax

Given a struct `Config` with 5 fields (all `String`), create a function that takes an existing `Config` and returns a new one where only the `api_key` is changed, but all other fields are the same, using the **struct update syntax (`..`)**.

### Question 6: Methods vs Associated Functions

Create a struct `Rectangle`. Implement an **associated function** `new(width: u32, height: u32) -> Rectangle` and a **method** `is_square(&self) -> bool`.

### Question 7: Exhaustive Matching

Create an enum `TrafficLight` with `Red`, `Yellow`, and `Green`. Write a function that returns the "next" light. Ensure that if a new color is added to the enum later, the code won't compile until the function is updated (i.e., don't use `_ =>`).

### Question 8: Destructuring Nested Enums

Create an enum `Message` that has a variant `Move { x: i32, y: i32 }`. Write a function that takes a `Message` and returns the sum of `x` and `y` only if it is a `Move` variant, otherwise return 0. Use `if let`.

### Question 9: The `Copy` Trait on Structs

Create a struct `Point { x: i32, y: i32 }`. Make this struct work with simple assignment (no moving) by deriving the necessary traits. Prove it works in a test where you assign `p1 = p2` and can still use `p2`.

### Question 10: Implementation Blocks for External Types

Can you implement a method for `Vec<i32>` directly in your crate? Explain why or why not (the "Orphan Rule") and provide a workaround using a "Newtype" pattern (a tuple struct wrapping the Vec).

## **Practice Sheet 3: Traits, Generics, and Common Traits**

This is where Rust’s "Power User" features live. We’re moving away from data storage and into **behavior**.

### **Question 1: The Basic Trait**

Define a trait `Summary` with a method `summarize(&self) -> String`. Implement this trait for two structs: `NewsArticle` and `Tweet`.
**File:** `src/sheet_3/q1.rs`

### **Question 2: Default Trait Implementations**

Update your `Summary` trait to have a default implementation: `"Read more..."`. Make it so `NewsArticle` uses the default, but `Tweet` overrides it with the actual tweet content.

### **Question 3: Trait Bounds (Generics)**

Write a function `notify<T: Summary>(item: &T)` that calls the `summarize` method and prints the result. This introduces you to generic constraints.
**File:** `src/sheet_3/q3.rs`

### **Question 4: Multiple Trait Bounds**

Write a function `compare_and_print<T>(item1: &T, item2: &T)` where `T` must implement both `Summary` AND `PartialEq`. The function should print "They are the same" if they match, otherwise print their summaries. Use the `where` clause syntax for readability.

### **Question 5: Returning Traits (`impl Trait`)**

Write a function `returns_summarizable() -> impl Summary`. Note the restriction: can you return _different_ types (e.g., sometimes a `Tweet`, sometimes an `Article`) using `impl Trait`? (Write your answer in a comment).

### **Question 6: The `Display` and `ToString` Relationship**

Implement the `std::fmt::Display` trait for a struct `Point { x: i32, y: i32 }`. Once implemented, show in a test case that you can now call `.to_string()` on a `Point` even though you didn't explicitly implement `ToString`.

- **Goal:** Understand how the standard library uses "Blanket Implementations."

### **Question 7: Custom Iterator**

Create a struct `Counter` that counts from 1 to 5. Implement the `Iterator` trait for it.
**File:** `src/sheet_3/q7.rs`

```rust
pub struct Counter { count: u32 }
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> { /* Your Logic */ }
}

```

### **Question 8: Generic Data Types**

Create a struct `Wrapper<T>` that holds a value of any type. Implement a method `value(&self) -> &T` and a method `new(val: T) -> Self`.

### **Question 9: From and Into Traits**

Implement `From<i32>` for a struct `Seconds(i32)`. Show that you can now use the `.into()` method to convert an `i32` into a `Seconds` struct. This is the idiomatic way to handle type conversions in Rust.

### **Question 10: Trait Objects (`dyn`)**

This is the "Boss" question of this sheet. Create a `Vec<Box<dyn Summary>>`. Fill it with a mix of `Tweet` and `NewsArticle` instances. Iterate through the vector and call `.summarize()` on each.

- **Goal:** Understand **Dynamic Dispatch** vs. **Static Dispatch**.

---

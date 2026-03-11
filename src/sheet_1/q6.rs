pub struct User {
    pub name: String,
    pub age: u32,
}

pub fn get_name_and_keep_user(user: User) -> (String, User) {
    let name_cpy = user.name.clone();
    (name_cpy, user)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_q6() {
        let u = User {
            name: String::from("Alice"),
            age: 30,
        };
        let (name, user_back) = get_name_and_keep_user(u);
        assert_eq!(name, "Alice");
        assert_eq!(user_back.age, 30);
    }
}

struct Profile {
    nickname: Option<String>,
}

struct User {
    profile: Option<Profile>,
}

fn get_nickname(user: &User) -> String {
    user.profile
        .as_ref()
        .and_then(|p| p.nickname.as_ref())
        .cloned()
        .unwrap_or(String::from("Guest"))
}

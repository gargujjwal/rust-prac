#[derive(Clone)]
struct Config {
    api_key: String,
    b: String,
    c: String,
    d: String,
    e: String,
}

fn update_api_key(c: &Config) -> Config {
    Config {
        api_key: String::from("new API KEY"),
        ..c.clone()
    }
}

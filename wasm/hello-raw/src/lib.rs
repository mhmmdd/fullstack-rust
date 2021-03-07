pub extern "C" fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

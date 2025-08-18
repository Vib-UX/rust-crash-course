pub fn hello() -> String {
    let s: String = "Hello Rust".to_string();
    return s;
}

pub fn greet(name: &str) -> String {
    let mut s: String = "Hello ".to_string();
    // can we use the + operator to concatenate strings?
    // push_str for string append, and push for char append
    s.push_str(name);
    return s;
}

pub fn append(mut s: String) -> String {
    let suffix: String = "!".to_string();
    // when &String it gets dereferenced to &str
    s+= &suffix;
    return s;
}

pub fn format_macro(name: &str) -> String {
    // we can use the format! macro to create a formatted string
    // it works like println! but returns a String
    let s: String = format!("Hello, {}!", name);
    return s;
}

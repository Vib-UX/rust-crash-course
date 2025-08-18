#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8,u8,u8,f32),
}

pub fn explain_enum() {
    // Option is an enum that can be either Some(T) or None
    // Result is an enum that can be either Ok(T) or Err(E)
    // Enums are used to define a type that can have multiple variants

    // When to use Option or when to use Result?
    // Use Option when you want to represent a value that may or may not be present
    // Use Result when you want to represent a value that may be successful or may fail

    // Eg. Option<i32> generally used in array traversal, in case pointer reaches out of bounds it returns None
    // Eg. Result<i32, String> can be used in string parsing, maybe we have to parse int -> string if able do that
    // return Ok(parsed_int) else return Err("Parsing failed".to_string())
}

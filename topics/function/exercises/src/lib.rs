pub fn mul(a: i32, b: i32) -> i32 {
    return a*b;
}

pub fn div(a: i32, b: i32) -> i32{
    if b == 0 {
       return 0; // Handle division by zero
    } else {
        return a / b;
    }

}

pub fn div_result(a: i32, b:i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

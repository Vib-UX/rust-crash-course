pub fn eq(a: char, b: char) -> bool{
    return a==b;
}

pub fn add(a: f32, b: f32, c: f32) -> f32 {
    return a+b+c;
}

pub fn cast(x: u8, y: i8, z: f32) -> f32 {
    // here we have to do type conversion
    return x as f32 + y as f32 + z;
}

pub fn min_max() -> Result<(i32, i32), String> {
    let max = i32::MAX;
    let min = i32::MIN;
    println!("Max: {}, Min: {}", max, min);
    return Ok((min,max));
}

pub fn overflow() -> Result<i32, String>{
    let max = i32::MAX;
    let result = max.checked_add(1);
    match result {
        Some(value) => Ok(value),
        None => Err("Overflow occurred".to_string()),
    }
}

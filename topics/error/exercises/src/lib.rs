#[derive(Debug, PartialEq)]
pub enum MathError{
    DivByZero
}

pub fn div(x: u32, y: u32) -> Result<u32, MathError> {
    let ans: Result<u32, MathError> = match y{
        0 => Err(MathError::DivByZero),
        _ => Ok(x/y),
    };

    return ans;
}

pub fn get(v: &[u32], i: usize, default_val: u32) -> u32 {
    let ans: u32 = match v.get(i){ // so get returns the pointer to the value at index i or None if it does not exist
       Some(val) => *val,
       None => default_val,
    };

    return ans;
}

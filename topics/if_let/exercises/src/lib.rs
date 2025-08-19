pub fn unwrap_or_default(x: Option<u32>, v: u32) -> u32 {
    if let Some(val) = x{ // why not double ==
        return val;
    }else{
        return v;
    }
}

pub fn unwrap_compare(x: Option<i32>, y: i32) -> i32 {
    if let Some(val)== x{ // what if we use == here?
        return val;
    }
}

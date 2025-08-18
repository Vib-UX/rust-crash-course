pub fn min(x: i32, y: i32) -> i32 {
    // can we write it in one line?
    if x< y{
        return x;
    }
    return y;
    
}

pub fn max(x: i32, y: i32) -> i32 {
    if x>y{
        return x;
    }
    return y;   
}

pub fn sign(x: i32) -> i32 {
    if x<0{
        return -1;
    }return 1;
}

pub fn init(x: u32, y: u32, z: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    v.push(x);
    v.push(y);
    v.push(z);
    return v;
}

pub fn init1(x: u32, y: u32, z: u32) -> Vec<u32> {
    let v: Vec<u32> = vec![x, y, z];
    return v;
}

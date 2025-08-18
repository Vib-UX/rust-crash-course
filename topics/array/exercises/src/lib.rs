pub fn zeros() -> [u32; 100] {
    // We predefine an array of 100 elements, all initialized to 0
    // This is a fixed-size array, not a vector
    // The type is [u32; 100], meaning an array of 100 u32 integers
    // The array is initialized with all elements set to 0
    // This is a compile-time constant, so the size must be known at compile time
    let arr: [u32;100] = [0; 100];
    arr
}

pub fn first_3(s: &[u32]) -> &[u32] {
    return &s[0..3];
}

pub fn last_3(s: &[u32]) -> &[u32] {
    // first lets fetch the length of the slice
    let len = s.len();
    // then we can return the last 3 elements
    // we can use the range syntax to get the last 3 elements
    return &s[len-3..len];
}

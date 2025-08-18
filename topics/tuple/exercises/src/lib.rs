pub fn first(t: (bool, u32, char)) -> bool {
    // have to return the first element of the tuple
    // can we use destructuring?
    let (first, _, _) = t;
    first
   
}

pub fn last(t: (bool, u32, char)) -> char {
    // have to return the last element of the tuple
    // can we use destructuring?
    let (_, _, last) = t;
    last
}

pub fn swap(t: (u32, u32)) -> (u32, u32) {
    // Swap the first and second elements of the tuple
    let (first, second) = t;
    // also update the tuple
    let mut swapped = t;
    swapped.0 = second;
    swapped.1 = first;
    swapped
}

pub fn unit_tuple() -> (){
    // Empty tuple = unit type
    // Result<(), String> = Ok(()) | Err("☠️")
}

pub fn exercise_1() {
    let s = "rust".to_string();
    let s1 = s;
    // let s2 = s; // this cannot be called since rule of ownership states there can only be one owner of a value
    println!("{s1}"); 
    // println!("{s2}"); 
}

pub fn exercise_2() {
    let s = "rust".to_string();
    {
        let s1 = s;
        println!("{s1}");
    }
    // as soon the scope ends, the ownership of s1 is dropped
    // println!("{s}");
}

fn take(s: String) {
    println!("take {s}");
}

pub fn exercise_3() {
    let s = "rust".to_string();
    take(s);
    // println!("{s}");
    // println!("{s}");
}

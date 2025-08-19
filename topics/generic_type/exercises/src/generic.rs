#[allow(unused)]

#[derive(Debug,PartialEq)]
pub struct Point<T>{
    x: T,
    y: T
}

fn swap<A,B>(a: A, b: B) -> (B,A){
    return (b,a);
}

fn main() {
    let p : Point<i32> = Point {x:0,y:0};
    println!("{:#?} ", p);
    
    let mut a: u32 =1;
    let mut b: i32=2;
    // (a,b) = swap(a,b); // even we are using mutatble we cant reassign the type
    let (a,b) = swap(a,b);
    println!("Swapped value of a: {}, Swapped value of b: {}", a,b);
   
}
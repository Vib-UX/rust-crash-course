#[allow(unused)]

#[derive(Debug,PartialEq)]
pub struct Point{
    x: f32,
    y: f32
}

// A method is simply a function that is attached to the data-type

impl Point {
    // Statis method -- associated functions
    fn new(x: f32, y: f32) -> Self{
        return Self{x,y};
    }
    // Method
    fn move_to(&mut self, x: f32, y: f32){
        self.x=x;
        self.y=y;
    }
}


fn main() {
    // let mut p : Point = Point {x:0.0,y:0.0};
    let mut p = Point::new(0.0,1.0);
    p.move_to(1.0,2.0);
    println!("{:#?} ", p);
    
  
   
}
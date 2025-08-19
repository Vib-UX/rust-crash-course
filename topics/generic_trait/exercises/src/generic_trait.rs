#![allow(unused)]

trait List<T>{
    fn count(&self) -> usize;
    fn first(&self) -> &T;
}

impl List<u32> for(u32, bool, char){
    fn count(&self) -> usize{
        3
    }
    
    fn first(&self) -> &u32{
        &self.0
    }
}

impl<T> List<T> for Vec<T>{
    fn count(&self) -> usize{
        return self.len();
    }
    
    fn first(&self) -> &T{
        match self.get(0){
            Some(val)=> val,
            None => panic!("vector is empty cannot find the first element")
        }
    }
}

fn main() {
    let tup: (u32, bool, char) = (42, true, 'a');
    println!("Tuple count: {}", tup.count());
    println!("Tuple first: {}", tup.first());

    let v = vec![10, 20, 30];
    println!("Vector count: {}", v.count());
    println!("Vector first: {}", v.first());

    // Uncommenting this will panic:
    // let empty: Vec<i32> = vec![];
    // println!("Empty vector first: {}", empty.first());
}

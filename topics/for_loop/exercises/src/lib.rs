pub fn sum(nums: Vec<i32>) -> i32 {
    let mut sum =0;
    for num in nums.iter(){
        sum+=num;
    }
    sum=0;
    // can i again run for loop?
    // only if we are calling iter() method by default it calls into_iter() which consumes the vector
    // so we can not use the same vector again
    // we can use iter() method to iterate over the vector without consuming it
    for num in nums.iter(){
        sum+=num;
    }

    return sum;
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
   let vec: Vec<u32> = vec![i; n];
   return vec;
}

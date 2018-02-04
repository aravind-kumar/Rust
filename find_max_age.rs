use std::io;
use std::{u64};

fn main() {
   let mut input_string = String::new();
   io::stdin().read_line(&mut input_string).ok().expect("Invalid number");
   let (mut count,mut max) = (0,u64::MIN);
   input_string = String::new();
   io::stdin().read_line(&mut input_string).ok().expect("Invalid line");
   for num in input_string.trim().split(" ") {
       let num : u64 = num.trim().parse().ok().expect("Invalid number parsing");
       if num > max {
           max = num;
           count = 1;
       } else if num == max {
           count+=1;
       }
    }
    println!("{}",count);
}

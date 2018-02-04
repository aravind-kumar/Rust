// Enter your code here 
use std::io;
use std::i64;

fn main() {
    let mut input_string = String::new();
    let (mut sum,mut min,mut max) = (0,i64::MAX,i64::MIN);
    io::stdin().read_line(&mut input_string).ok().expect("Invalid line");
    for num in input_string.trim().split(" ") {
        let val = num.trim().parse().ok().expect("Invalid number");
        if val < min {
            min = val;
        } if val > max {
            max = val;
        }
        sum+=val;
    }
    println!("{} {}",sum-max,sum-min);
}

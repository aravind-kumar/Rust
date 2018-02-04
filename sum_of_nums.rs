use std::io;
fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).ok().expect("Invalid input1");
    let mut given_nums = String::new();
    io::stdin().read_line(&mut given_nums).ok().expect("Invalid line");
    let mut sum : i32 = 0;
    for num in given_nums.split(" ") {
        let num_int : i32 = num.trim().parse().ok().expect("Invalid number");
        sum=sum+num_int;
    }
    println!("{}",sum);
}

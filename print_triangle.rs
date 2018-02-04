// Enter your code here 
use std::io;

fn main() {
    let mut input_num = String::new();
    io::stdin().read_line(&mut input_num).ok().expect("Invalid input");
    let input_num: usize = input_num.trim().parse().ok().expect("Invalid number");
    let mut i = 1;
    while i<=input_num {
        let s = format!("{:#>1$}","",i);
        println!("{}",s);
        i+=1;
    }
}

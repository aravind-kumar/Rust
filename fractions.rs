// Enter your code here 
use std::io;

fn main() {
    
    let mut num = String::new();
    io::stdin().read_line(&mut num).ok().expect("Invalid input");
    let num:f32 = num.trim().parse().ok().expect("Invalid input");
    
    let (mut positive,mut negative,mut zero) = (0.0,0.0,0.0);
    
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).ok().expect("Invalid line");
    for num in input_line.trim().split(" ") {
        let num:i32 = num.trim().parse().ok().expect("Invalid number");
        match num {
            num if num > 0 => positive+=1.0,
            num if num < 0 => negative+=1.0,
            num if num == 0 => zero+=1.0,
            _ => println!("Invalid input"),
        }
    }
    println!("{:.6}",positive/num);
    println!("{:.6}",negative/num);
    println!("{:.6}",zero/num);
    
}

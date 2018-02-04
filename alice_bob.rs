// Enter your code here 

use std::io;

fn main() {
    
    let mut input_string=String::new();
    let mut alice = Vec::new();
    
    io::stdin().read_line(&mut input_string).ok().expect("Invalid line");

    for num in input_string.trim().split(" ") {
        let num1:i32 = num.trim().parse().ok().expect("Invalid input");
        alice.push(num1);
    }
    
    let mut bob = Vec::new();
    input_string = String::new();
    io::stdin().read_line(&mut input_string).ok().expect("Invalid line");
    for num in input_string.trim().split(" ") {
        let num1:i32 = num.trim().parse().ok().expect("Invalid input");
        bob.push(num1);
    }
    
    let mut i = 0;
    let mut acount = 0;
    let mut bcount = 0;
    while i<3 {
        if alice[i] > bob[i] {
            acount+=1;
        } else if bob[i] > alice[i] {
            bcount+=1;
        }
        i+=1;
    }
    println!("{0} {1}",acount,bcount);

}

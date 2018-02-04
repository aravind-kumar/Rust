// Enter your code here 
use std::io;

fn main() {
    
    let mut input_num = String::new();
    io::stdin().read_line(&mut input_num).ok().expect("Invalid input1");
    let num = input_num.trim().parse().ok().expect("Invalid number");
    let mut matix : Vec<Vec<i32>> = vec![vec![0;num];num];
    let mut i = 0;
    while i<num {
        let mut input_line = String::new();
        let mut j = 0;
        io::stdin().read_line(&mut input_line).ok().expect("Invalid line");
        for num in input_line.trim().split(" ") {
            let given_num : i32 = num.trim().parse().ok().expect("Invalid input");
            matix[i][j] = given_num;
            j+=1;
        }
        i+=1;
    }
    i = 0;
    let mut sum1 = 0;
    let mut sum2 = 0;
    while i<num {
        let mut j = 0;
        while j<num {
            if i == j {
                sum1 += matix[i][j];
            } 
            if i+j == num-1 {
                sum2 += matix[i][j];
            }
            j+=1;
        }
        i+=1;
    }
    let diff = sum1-sum2;
    println!("{}",diff.abs());

}

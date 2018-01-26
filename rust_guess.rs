extern crate rand;

use std::io;
use rand::Rng;

fn main() {
 
   let rand_num = rand::thread_rng().gen_range(1,100);
   println!("The secret number is {}",rand_num);
   println!("Enter the guess number");
   let mut guess_num = String::new();
   io::stdin().read_line(&mut guess_num).expect("Please enter a valid value");
   println!("The guess is {}",guess_num);
}

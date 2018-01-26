

use std::io;

fn main() {

   println!("Enter the guess number");
   let mut guess_num = String::new();
   io::stdin().read_line(&mut guess_num).expect("Please enter a valid value");
   println!("The guess is {}",guess_num);
}

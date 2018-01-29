

fn main() {

   let guess_num :u32 = 100;
   let name = "Aravind";
   println!("Enter the name");
   println!("We are using indexed parametrs to print the following");
   println!("My name is {0} and i score {1} out of {1}",name,guess_num);
   println!("Print using named arguments");
   println!("my name is {name} and i score {score} out of {score}",name=name,score=guess_num);  
   println!("The guess number in hex is {:x}", guess_num);
   println!("The guess number in binary is {:b}", guess_num);
   println!("The guess number in octal is {:o}", guess_num);

}

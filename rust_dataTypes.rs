use std::{i8,i16,i32,i64,u8,u16,u32,u64,isize,usize,f32,f64}; //buildin datatypes


fn main() {

   println!("Hello world");
  
   let _temp =10; // Type inference 
   let _temp1 : i32 = 100; // With explicit type definition
   let _is_temp_position : bool = _temp > 1; //Boolean type
   let _most_freq_letter : char = 'a'; //Character type
   let (a,b) = (5,3); // Assigning multiple values 
   
   println!("The value of A is {} ", a);
   println!("The value of B is {} ", b);
   println!("The max is {} ", i8::MAX);
   println!("The max is {} ", i16::MAX);
   println!("The max is {} ", i32::MAX);
   println!("The max is {} ", i64::MAX);
   println!("The max is {} ", u8::MAX);
   println!("The max is {} ", u16::MAX);
   println!("The max is {} ", u32::MAX);
   println!("The max is {} ", u64::MAX);
   println!("The max is {} ", isize::MAX);
   println!("The max is {} ", usize::MAX);
   println!("The max is {} ", f32::MAX);
   println!("The max is {} ", f64::MAX);

   println!("The max is {} ", i8::MIN);
   println!("The max is {} ", i16::MIN);
   println!("The max is {} ", i32::MIN);
   println!("The max is {} ", i64::MIN);
   println!("The max is {} ", u8::MIN);
   println!("The max is {} ", u16::MIN);
   println!("The max is {} ", u32::MIN);
   println!("The max is {} ", u64::MIN);
   println!("The max is {} ", isize::MIN);
   println!("The max is {} ", usize::MIN);
   println!("The max is {} ", f32::MIN);
   println!("The max is {} ", f64::MIN);

}

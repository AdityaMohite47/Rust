use std::io;

fn main() {
    println!("::Calculator::");

    println!("Enter the Operation:");
    
    let mut op = String::new();
    
    io::stdin()
	.read_line(&mut op)
	.expect("Failed to read line");

   let mut a = String::new();
   println!("Enter first number:");
   io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");
   
   let mut b = String::new();
   println!("Enter second number:");
   io::stdin()
        .read_line(&mut b)
        .expect("Failed to read line");
   

  println!("a:{} , b:{}", a , b );
}

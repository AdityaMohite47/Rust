fn main() {

    // standard if 
    if true {
    	println!("Hello World!");
    }
    
    // simple if-elseif-else chain
    let num = 3;
    
    if num < 4 {
	println!("num less than 4");
    }else if num > 4 {
	println!("num greater than 4");
    }else {
	println!("num equal to 4");
    }
   
    // single line if-else
    let condition = true;
    let number = if condition {1} else {0};

    println!("Value of number is {}" , number);


    // Rust has 3 loop types
    // 1. loop {} - without break it is an infinite loop
    let mut counter = 0;
    
    // loop with a exit condition and return value.
    let result = loop {
      counter +=1 ;
      
      if counter == 10 {
        break counter * 2;
      }
    };
    println!("count : {}" , result);
    

}

fn main() {
    println!("Hello World!");
    dio();
    let num = five();
    println!("2 + {} is:{}", num , add(2 , num));
}

// simple function
fn dio(){
	println!("ZA WARDOO!!");
}

// function with parameters and return value
fn add(a:i32 , b:i32) -> i32{
	return a + b;
}

// another way to return values from function
fn five() -> i32 {
 5
}

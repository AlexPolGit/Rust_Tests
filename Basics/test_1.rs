fn main()
{
	println!("Hello World!");
	
	let x: i32 = 5; //declare a constant 32-bit integer x to be 5
	let mut y = 10;
	let z = x + y;
	
	y = 20;
	
	let (a, b) = (1, 2);
	
	println!("x = {}, y = {}, z = x + y = {}.", x, y, z); //prints the value of x
	
	{
		let num = 256;
		let y = -64;
		
		println!("Also, num = {} and a different y is {}, num * y = {}", num, y, num * y);
	}
	
	printInt32(x);
	println!("5 + 1 = {}", addOne(5));
	
	let mut array1: [i32; 5];
	array1 = [12, 6000, 53, 8, -61];
	
	println!("I have an array of: {}, {}, {}, {} and {}", array1[0], array1[1], array1[2], array1[3], array1[4]);
	
	
}

fn printInt32(x: i32)
{
	println!("Number printed with a function: {}", x);
}

fn addOne(x: i32) -> i32
{
	x + 1
}
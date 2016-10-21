use std::io;

fn main()
{
	let mut initialNumArray: [f64; 5];
	let mut num = 0.0;
	println!("Enter 5 numbers: ");
	
	for(i, v) in (5..10).enumerate()
	{
		print!("{}: ", i);
		//io::stdin().read_line(&mut num);
		//initialNumArray[v] = num;
	}
	/*
	println!("Your array was: {}, {}, {}, {}, {}.", initialNumArray[0], initialNumArray[1], initialNumArray[2], initialNumArray[3], initialNumArray[4]);
	*/
}
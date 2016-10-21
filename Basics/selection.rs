fn main()
{
	let mut x = 5;
	
	let y = if x < 0
	{
		-1
	}
	else if x > 0
	{
		1
	}
	else
	{
		0
	};

	println!("x is {}, y is {}.", x, y);
	print!("Therefore x is ");
	
	if y == 1
	{
		println!("positive.");
	}
	else if y == -1
	{
		println!("negative.");
	}
	else
	{
		println!("neither positive or negative.");
	}
}
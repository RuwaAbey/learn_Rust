use std::io;

fn main() {
    
	println!("Enter the fibonacci number: ");
	
	let mut number = String::new();

	io::stdin().read_line(&mut number).expect("Failed to read line");

	let number: u32 = match number.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			println!("Please enter a number");
			return;
		}
	};
	
	let mut curr = 0;
	let mut next = 0;
	let mut prev = 0;

	for num in 0..(number + 1){
		if num == 0 {
			curr = 0;
		} else if num == 1 {
			curr = 1;
			prev = 0;
		}else {
			next = prev + curr;
			prev = curr;
			curr = next;	
			
		}
	}

	println!("Fibonacci value: {curr}");

	
}

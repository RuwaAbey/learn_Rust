use std::io;

fn main() {
    println!("Hi Welcome to temperature converter!");

    println!("Type C to convert temp from Celsius to Fahrenheit");
    println!("Type F to convert temp from Fahrenheit to Celsius");

    let mut unit = String::new();

    io::stdin().read_line(&mut unit).expect("Failed to read line");
    
    println!("Enter the temperature value");
	
    let mut value = String::new();

    io::stdin().read_line(&mut value).expect("Failed to read line");

    let mut value: u32 = match value.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Invalid temperature!");
				return;
			} 
		};

    match unit.trim() {
	"C" => {
             println!("You choose Celsius");
	     value = value * (9/5) + 32;
	     println!("Temperature in Celsius: {value}");
               }
	"F" => { 
	     println!("You choose Fahrenheit");
	     value = (value - 32) * (5/9);
	     println!("Temperature in Fahrenheit: {value}");
		}
	_ => println!("Invalid Input")
	}
}

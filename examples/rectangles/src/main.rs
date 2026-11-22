struct Rectangle {
	width: u32,
	height: u32,
}

fn main() {
	let width1 = 30;
	let height1 = 50;
	
	println!("The are1 of the rectangle is {} suqare pixels", area1(width1, height1));

        let rect1 = (30, 50);
	
	println!("The area2 of the rectangle is {} square pixels", area2(rect1));	

	//Refractoring with structs
	let rectangle = Rectangle {
		width: 30,
		height: 50,
	};

	println!("The are3 of the rectangle is {} square pixels", area3(&rectangle));	
	
}

fn area1(width: u32, height: u32) -> u32{
	width * height
}

fn area2(dimentions: (u32,u32)) -> u32 {
	dimentions.0 * dimentions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}

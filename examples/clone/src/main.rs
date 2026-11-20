fn main() {
	let first = String::from("Ferris");
	let first_clone = first.clone();
	let full = add_sufix(first_clone);
	println!("{first} changed to {full}");
	

}

fn add_sufix(mut name: String) -> String {
	name.push_str("Jr");
	name
}

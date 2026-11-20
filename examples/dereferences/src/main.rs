fn main() {
	
	let mut x: Box<i32> = Box::new(1);
	let a: i32 = *x;
	*x + 1;

	println!("{}", a);
	
	let r1: &Box<i32> = &x;
	let b: i32 = **r1;

	println!("{}", b);

	let r2: &i32 = &*x;
	let c: i32 = *r2;

	println!("{}", c);

	let x: Box<i32> = Box::new(-1);
	let x_abs1 = i32::abs(*x);
	let x_abs2 = x.abs();
	assert_eq!(x_abs1, x_abs2);

	let r: &Box<i32> = &x;
	let r_abs1 = i32::abs(**r);
	let r_abs2 = r.abs();
	assert_eq!(r_abs1, r_abs2);

	let s = String::from("Hello");
	let s_len1 = str::len(&s);
	let s_len2 = s.len();
	assert_eq!(s_len1, s_len2);
}

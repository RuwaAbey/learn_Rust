struct Cli {
	pattern: String,
	path: std::path::PathBuf,
}

fn main() {
	// standard library contains the function std::env::args() that gives you an iterator of the given arguements	
	let pattern = std::env::args().nth(1).expect("no pattern given");
	let path = std::env::args().nth(2).expect("no path given");

	let args = Cli {
		pattern,
		path: std::path::PathBuf::from(path), //PathBuf is like a String but for file system paths that work cross-platform
	};

	
	println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}

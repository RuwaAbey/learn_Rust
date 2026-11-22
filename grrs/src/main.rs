use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
	/// The pattern to look for
	pattern: String,
	/// The path to the file to read
	path: std::path::PathBuf,
}

fn main() {
	let args = Cli::parse();
	println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
	
	let result = File::open(args.path);
	let content  = match result {
		Ok(content) => { content },
		Err(error) => { panic!("Can't dealt with {}, just exit here", error); }
	};

	let content = BufReader::new(content);

	for line in content.lines() {
		let line = line.unwrap();
		if line.contains(&args.pattern) {
			println!("{}",line);
		}
	}
}

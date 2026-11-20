struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}



fn main() {
	
	let user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
		
	};

	let mut user2 = User {
		email: String::from("none@example.com"),
		username: String::from("noneusername123"),
		active: true,
		sign_in_count: 2,
	}; 

	user2.email = String::from("anotheremail@example.com");

	let mut user3 = build_user("user3@example.com".to_string(), "user3".to_string());

}

fn build_user(email: String, username: String) -> User {
	User {
		active: true,
		username: username,
		email: email,
		sign_in_count: 1,
	}
}

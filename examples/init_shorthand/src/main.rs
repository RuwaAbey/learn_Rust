struct User {
	active:bool,
	username: String,
	email: String,
	sign_in_count: u64
}

fn main() {

	let mut user1 = build_user("user1@example.com".to_string(),"user1".to_string());

	let user2 = User {
		active: user1.active,
		username: user1.username.clone(),
		email: String::from("another@example.com"),
		sign_in_count: user1.sign_in_count,
	};

	let user3 = User {
		email: String::from("user3@example.com"),
		..user1
	};	
}

fn build_user(email: String, username: String) -> User{
	User {
		active: true,
		username,
		email,
		sign_in_count: 1,
	}
}


use user::User;

pub struct Message {
	text: String,
	user: User
}

impl Message {
	pub fn text(&self) -> &String {
		&self.text
	}

	pub fn user(&self) -> &User {
		&self.user
	}
}


use user::User;

pub struct ChatMessage {
	text: String,
	user: User
}

impl ChatMessage {
	pub fn text(&self) -> &String {
		&self.text
	}

	pub fn user(&self) -> &User {
		&self.user
	}
}

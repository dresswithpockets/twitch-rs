
use std::collections::HashMap;

pub struct User {
	id: u64,
	display_name: String
}

impl User {
	pub fn from(id: u64, display: String) -> User {
		User {
			id: id,
			display_name: display
		}
	}

	pub fn id(&self) -> &u64 {
		&self.id
	}

	pub fn display_name(&self) -> &String {
		&self.display_name
	}
}

pub struct UserState {
	badges: HashMap<String, String>,
	color: String,
	display: String,
	emotes: String,
	channel: String,
	subscriber: bool,
	moderator: bool,
	user_type: UserType
}

impl UserState {
	pub fn from_irc(irc: &String) -> UserState {
		// TODO: determine UserState from IRC message
		unimplemented!()
	}
}

pub enum UserType {
	Viewer,
	Moderator,
	GlobalModerator,
	Broadcaster,
	Admin,
	Staff
}

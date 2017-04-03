
use message::Message;

pub struct TwitchClient {
	pub user: String,
	pub auth: String,
	pub default_channel: String,
	pub logging: bool,
}

pub enum Event {
	MessageReceived(Message),
}

impl TwitchClient {
	pub fn connect(&self) {
		// TODO: Connect!
	}

	pub fn recv_event(&self) -> Result<Event, &str> {

		Err("Not yet implemented")
	}
}

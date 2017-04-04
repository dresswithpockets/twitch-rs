

use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode, Error};
use channel;
use message;
use std::error;

static DEFAULT_HOST: &'static str = "irc-ws.chat.twitch.tv";
const DEFAULT_PORT: i32 = 80;

pub struct TwitchClient {
	credentials: ConnectionCredentials,
	default_channel: String,
	logging: bool,

	channels: Vec<channel::Channel>,
}

impl TwitchClient {
	pub fn from(user: String, auth: String, channel: String, log: bool) -> TwitchClient {

		// TODO: add logging capabilities

		// TODO: add command identifiers for chat and whispers

		TwitchClient {
			credentials: ConnectionCredentials::from(user, auth),
			default_channel: channel,
			logging: log,

			channels: Vec::new(),
		}
	}

	pub fn connect(&self) {
		connect(format!("ws://{}:{}", self.credentials().host(), self.credentials().port()),
			|out| {
				WebClient {
					out: out
				}
			}
		);
	}

	pub fn recv_event(&self) -> Result<Event> {

		// TODO: return next event in queue
		Ok(Event::None)
	}

	pub fn credentials(&self) -> &ConnectionCredentials {
		&self.credentials
	}

	fn parse_irc(irc: String) {
		// TODO: handle irc commands here

		// On Message Received
	}
}

pub struct WebClient {

	out: Sender,
}

impl Handler for WebClient {

	fn on_open(&mut self, _: Handshake) -> Result<()> {

		// TODO: handle on open garbo
		Ok(())
	}

	fn on_message(&mut self, msg: Message) -> Result<()> {

		// TODO: handle on message garbo
		Ok(())
	}

	fn on_close(&mut self, code: CloseCode, reason: &str) {

		// TODO: handle on close garbo
	}

	fn on_error(&mut self, err: Error) {
		// TODO: handle error garbo
	}
}

pub struct ConnectionCredentials {
	user: String,
	auth: String,
	host: String,
	port: i32,
}

impl ConnectionCredentials {

	pub fn from(user: String, auth: String) -> ConnectionCredentials {
		ConnectionCredentials::from_host(user, auth, DEFAULT_HOST.to_owned(), DEFAULT_PORT)
	}

	pub fn from_host(
		user: String,
		auth: String,
		host: String,
		port: i32
	) -> ConnectionCredentials {

		ConnectionCredentials {
			user: user,
			auth: auth,
			host: host,
			port: port,
		}
	}

	pub fn user(&self) -> &String {
		&self.user
	}

	pub fn auth(&self) -> &String {
		&self.auth
	}

	pub fn host(&self) -> &String {
		&self.host
	}

	pub fn port(&self) -> &i32 {
		&self.port
	}
}

pub enum Event {
	None,

	// TODO: add all event types

	MessageReceived(message::Message),
}

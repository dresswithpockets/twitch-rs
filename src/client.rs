
use ws;

use channel;
use message;
use rfc;
use std::error;

static DEFAULT_HOST: &'static str = "irc-ws.chat.twitch.tv";
const DEFAULT_PORT: i32 = 80;

pub struct TwitchClient {

	out: ws::Sender,

	credentials: ConnectionCredentials,
	default_channel: String,
	logging: bool,

	channels: Vec<channel::Channel>,
}

impl TwitchClient {
	pub fn start(user: String, auth: String, channel: String, log: bool) -> ws::Result<()> {

		// TODO: add logging capabilities

		// TODO: add command identifiers for chat and whispers

		let credentials = ConnectionCredentials::from(user, auth);
		//let client: TwitchClient;


		// TODO: Setup websocket connection
		/*ws::connect(
			format!("wss://{}:{}", credentials.host(), credentials.port()),
			move |out| {
				TwitchClient {
					out: out,

					credentials: credentials,
					default_channel: channel,
					logging: log,

					channels: Vec::new(),
				}
			}
		).unwrap();*/

		Ok(())

		/*TwitchClient {
			credentials: credentials,
			default_channel: channel,
			logging: log,

			channels: Vec::new(),
		}*/
	}

	pub fn credentials(&self) -> &ConnectionCredentials {
		&self.credentials
	}

	pub fn connect(&self) {
		/*connect(format!("ws://{}:{}", self.credentials().host(), self.credentials().port()),
			|out| {
				WebClient {
					out: out,
					/*open_func: |wc: &mut WebClient, hs: Handshake| -> Result<()> {
						self.on_open(wc, hs)
					},

					close_func: |wc: &mut WebClient, cc: CloseCode, reason: &str| {
						self.on_close(wc, cc, reason);
					},

					message_func: |wc: &mut WebClient, msg: Message| -> Result<()> {
						self.on_message(wc, msg)
					},

					error_func: |wc: &mut WebClient, err: Error| {
						self.on_error(wc, err);
					},*/
				}
			}
		);*/
	}

	pub fn disconnect(&self) {
		// TODO: Disconnect!
	}

	pub fn recv_event(&self) -> Result<Event, &str> {

		// TODO: return next event in queue
		Ok(Event::None)
	}

	pub fn web_send(&self, msg: String) -> ws::Result<()> {
		self.out.send(msg)
	}

	fn parse_irc(irc: String) {
		// TODO: handle irc commands here

		// On Message Received
	}

	fn on_web_open(&self, hs: ws::Handshake) -> ws::Result<()> {

		self.web_send(
			rfc::pass(self.credentials().auth())
		)?;
		self.web_send(
			rfc::nick(self.credentials().user())
		)?;
		self.web_send(
			rfc::user(
				self.credentials().user(),
				&0,
				self.credentials().user()
			)
		)?;

		self.web_send(String::from("CAP REQ twitch.tv/membership"))?;
		self.web_send(String::from("CAP REQ twitch.tv/commands"))?;
		self.web_send(String::from("CAP REQ twitch.tv/tags"))?;

		if !self.default_channel.is_empty() {
			// TODO: Join default channel
		}

		Ok(())
	}

	fn on_web_message(&self, msg: ws::Message) -> ws::Result<()> {
		Ok(())
	}

	fn on_web_close(&self, code: ws::CloseCode, reason: &str) {

	}

	fn on_web_error(&self, err: ws::Error) {

	}
}


impl ws::Handler for TwitchClient {

	fn on_open(&mut self, hs: ws::Handshake) -> ws::Result<()> {
		self.on_web_open(hs)
	}

	fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
		self.on_web_message(msg)
	}

	fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
		self.on_web_close(code, reason)
	}

	fn on_error(&mut self, err: ws::Error) {
		self.on_web_error(err)
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

		// we're offloading this oauth fix here
		// since it doesnt need to be in on_open/OnConnected

		let mut auth = auth;

		if !auth.contains(":") {
			auth = format!("oauth:{}", auth.replace("oauth", ""));
		}

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
	Connected(String, String), // username, default_channel
	MessageReceived(message::ChatMessage), // chat_message
}

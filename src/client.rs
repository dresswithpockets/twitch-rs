
use ws;

use url::Url;

use std::error::Error;
use std::{thread, time};

use user;
use channel;
use message;
use rfc;
use irc;
use subscriber;

static DEFAULT_HOST: &'static str = "irc-ws.chat.twitch.tv";
const DEFAULT_PORT: i32 = 80;

pub struct TwitchClient {
	out: ws::Sender,

	credentials: ConnectionCredentials,
	default_channel: String,
	logging: bool,

	channels: Vec<channel::Channel>,

	on_event: fn(&TwitchClient, Event),
}

impl TwitchClient {
	pub fn connect(
		user: String,
		auth: String,
		channel: String,
		log: bool,
		on_event: fn(&TwitchClient, Event),
	) -> ws::Result<()> {

		// TODO: add logging capabilities

		// TODO: add command identifiers for chat and whispers

		let creds = ConnectionCredentials::from(user, auth);

		ws::connect(
			format!("wss://{}:{}", creds.host(), creds.port()),
			|out| {
				TwitchClient {
					out: out,

					credentials: creds.clone(),
					default_channel: channel.clone(),
					logging: log.clone(),

					channels: Vec::new(),

					on_event: on_event,
				}
			}
		);

		Ok(())
	}

	pub fn credentials(&self) -> &ConnectionCredentials {
		&self.credentials
	}

	pub fn web_send(&self, msg: String) -> ws::Result<()> {
		self.out.send(msg)
	}

	pub fn parse_irc(&self, irc: String) {

		if irc::detect_connected(&irc) {
			(self.on_event)(self, Event::Connected(
				self.credentials().user().clone(),
				self.default_channel.clone()
			));
			return;
		}

		match irc::detect_new_subscriber(&irc, &self.channels) {
			Some(channel) => {
				(self.on_event)(self, Event::NewSubscriber(
					channel,
					subscriber::NewSubscriber::from(irc)
				));
				return;
			},
			_ => {}
		}

		// TODO: handle irc commands here
	}

	pub fn join_channel(&self, channel: String) {
		// TODO: Join channel
		unimplemented!()
	}

	pub fn send_jc_message(&self, channel: &channel::Channel, message: &String) {
		// TODO: Check if messages are throttled (refer to TwitchClient.cs:297)

		self.web_send(format!(
			":{user}!{user}@{user}.tmi.twitch.tv PRIVMSG #{chan} :{msg}",
			user = self.credentials().user(),
			chan = channel.name(),
			msg = message
		));

	}

	pub fn send_channel_message(&self, channel: &String, message: &String) {
		self.send_jc_message(self.channel_from_string(channel).unwrap(), message);
	}

	pub fn send_message(&self, message: &String) {
		match self.channels.first() {
			Some(channel) => self.send_jc_message(channel, message),
			None => {}
		};
	}

	pub fn send_whisper(&self, receiver: String, message: String) {

		// TODO: Check if whispers are throttled (refer to TwitchClient.cs:333)

		self.web_send(format!(
			":{user}~{user}@{user}.tmi.twitch.tv PRIVMSG #jtv :/w {r} {m}",
			user = self.credentials().user(),
			r = receiver,
			m = message
		));
		(self.on_event)(self, Event::WhisperSent(
			receiver,
			message
		));
	}

	pub fn channel_from_string(&self, channel: &String) -> Result<&channel::Channel, ()> {
		for (index, chan) in self.channels.iter().enumerate() {
			if chan.name().to_lowercase() == channel.to_lowercase() {
				return Ok(&self.channels[index]);
			}
		}
		Err(())
	}

	pub fn disconnect(&mut self) -> ws::Result<()> {
		self.log(String::from("Disconnect Twitch Chat Client..."));

		self.out.close(ws::CloseCode::Abnormal)?;

		self.channels.clear();

		Ok(())
	}

	pub fn reconnect(&self) -> ws::Result<()> {
		self.log(format!(
			"Reconnecting to: {}:{}",
			self.credentials().host(),
			self.credentials().port()
		));

		self.out.close(ws::CloseCode::Abnormal);
		let url = Url::parse(format!(
			"wss://{}:{}",
			self.credentials().host().clone(),
			self.credentials().port().clone()
		).as_str()).unwrap();
		self.out.connect(url)
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
			self.join_channel(self.default_channel.clone())
		}

		Ok(())
	}

	fn on_web_message(&self, msg: ws::Message) -> ws::Result<()> {

		let is_text = msg.is_text();
		let data = msg.into_text().unwrap_or(String::from(""));
		let lines = data.split("\r\n");

		for line in lines {
			let line = String::from(line);
			if line.len() > 1 {
				self.log(format!("Received: {}", line));
				if is_text {
					(self.on_event)(self, Event::SendReceiveData(
						SendReceiveDirection::Received,
						line.clone()
					));
					self.parse_irc(line);
				}
			}
		}


		Ok(())
	}

	fn on_web_close(&mut self, code: ws::CloseCode, reason: &str) {
		(self.on_event)(self, Event::Disconnected(
			self.credentials().user().clone(),
			reason.to_owned()
		));
		self.channels.clear();
	}

	fn on_web_error(&self, err: ws::Error) {

		self.reconnect();

		thread::sleep(time::Duration::from_secs(2));

		(self.on_event)(self, Event::ConnectionError(
			self.credentials().user().clone(),
			err.description().to_owned()
		));
	}

	fn log(&self, message: String) {
		if self.logging  {
			// TODO: Implement logging
		}
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

impl Clone for ConnectionCredentials {
	fn clone(&self) -> ConnectionCredentials {
		ConnectionCredentials {
			user: self.user.clone(),
			auth: self.auth.clone(),
			host: self.host.clone(),
			port: self.port.clone()
		}
	}
}

pub enum Event {
	None,

	// TODO: add all event types

	Connected(String, String), // username, default_channel
	ChannelJoined(String, String), // username, channel
	IncorrectLogin(String), // error message
	ChannelStateChanged(channel::ChannelState, String), // channel_state, channel
	UserStateChanged(user::UserState), // user_state
	MessageReceived(message::ChatMessage), // chat_message

	WhisperSent(String, String), // receiver, message

	NewSubscriber(String, subscriber::NewSubscriber), // channel, new_sub

	Disconnected(String, String), // username, reason
	ConnectionError(String, String), // username, message

	SendReceiveData(SendReceiveDirection, String), // direction, data
}

pub enum SendReceiveDirection {
	Sent,
	Received,
}

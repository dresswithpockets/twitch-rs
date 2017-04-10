/*
Tristen Horton
tristen@tristenhorton.com
2017-04-03

Module for basic channel data
Its possible to be in multiple channels at once in Twitch, so we want a simple way
to represent multiple channels at once - we'll be using this module and structure
for that.
*/

use message::ChatMessage;
use std::str::Split;

pub struct Channel {
	name: String,
	state: Option<ChannelState>
}

impl Channel {

	pub fn name(&self) -> &str {
		self.name.as_str()
	}

	pub fn state(&self) -> Option<ChannelState> {
		self.state
	}
}

impl<'a> From<&'a str> for Channel {
	fn from(name: &'a str) -> Channel {
		Channel {
			name: name.to_string(),
			state: None
		}
	}
}

impl Clone for Channel {
	fn clone(&self) -> Channel {

		Channel {
			name: self.name,
			state: match self.state {
				Some(ref st) => {
					Some(st.clone())
				}
				_ => { None }
			}
		}
	}
}

pub struct ChannelState {

	r9k_mode: bool,
	sub_only: bool,
	slow_mode: bool,
	emote_only: bool,

	channel: String,
	broadcaster_lang: String,
}

impl ChannelState {
	pub fn from(irc: &str) -> ChannelState {

	// typical irc message:
	// broadcaster-lang=;emote-only=0;r9k=0;slow=0;subs-only=1 :tmi.twitch.tv ROOMSTATE #phxvyper

		let properties = irc.split_whitespace().nth(0);
		let mut r9k = false;
		let mut sub = false;
		let mut slow = false;
		let mut emote = false;
		let mut lang = "";
		let mut channel;

		for prop in properties.unwrap_or("").split(";") {
			let mut prop_split = prop.split("=");
			match prop_split.nth(0).unwrap_or("") {

				"broadcaster-lang" => lang = prop_split.nth(1).unwrap_or(""),

				"emote-only" => emote = bool_from_nth_str(&mut prop_split, 1),
				"r9k" => r9k = bool_from_nth_str(&mut prop_split, 1),
				"slow" => slow = bool_from_nth_str(&mut prop_split, 1),
				"subs-only" => sub = bool_from_nth_str(&mut prop_split, 1),

				_ => println!("[twitch-rs] Unnacounted for irc property: {:?}", prop),
			}
		}

		channel = irc.split("#").nth(1).unwrap_or("");

		ChannelState {
			r9k_mode: r9k,
			sub_only: sub,
			slow_mode: slow,
			emote_only: emote,
			channel: channel.to_string(),
			broadcaster_lang: lang.to_string(),
		}
	}
}

impl Clone for ChannelState {
	fn clone(&self) -> ChannelState {

		ChannelState {
			r9k_mode: self.r9k_mode,
			sub_only: self.sub_only,
			slow_mode: self.slow_mode,
			emote_only: self.emote_only,

			channel: self.channel,
			broadcaster_lang: self.broadcaster_lang,
		}
	}
}

fn bool_from_nth_str(split: &mut Split<&str>, nth: usize) -> bool {
	split.nth(nth)
		.unwrap_or("false")
		.parse::<bool>()
		.unwrap_or(false)
}

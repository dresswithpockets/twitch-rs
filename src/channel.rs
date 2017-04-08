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
	pub fn from(name: String) -> Channel {
		Channel {
			name: name,
			state: None,
		}
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn state(&self) -> &Option<ChannelState> {
		&self.state
	}
}

impl Clone for Channel {
	fn clone(&self) -> Channel {

		Channel {
			name: self.name.clone(),
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
	pub fn from(irc: String) -> ChannelState {

	// typical irc message:
	// broadcaster-lang=;emote-only=0;r9k=0;slow=0;subs-only=1 :tmi.twitch.tv ROOMSTATE #phxvyper

		let properties = irc.split_whitespace().nth(0);
		let mut r9k = false;
		let mut sub = false;
		let mut slow = false;
		let mut emote = false;
		let mut lang = String::from("");
		let mut channel;

		for prop in properties.unwrap_or("").split(";") {
			let mut prop_split = prop.split("=");
			match prop_split.nth(0).unwrap_or("") {

				"broadcaster-lang" => lang = prop_split.nth(1).unwrap_or("").to_string(),

				"emote-only" => emote = bool_from_nth_str(&mut prop_split, 1),
				"r9k" => r9k = bool_from_nth_str(&mut prop_split, 1),
				"slow" => slow = bool_from_nth_str(&mut prop_split, 1),
				"subs-only" => sub = bool_from_nth_str(&mut prop_split, 1),

				_ => println!("[twitch-rs] Unnacounted for irc property: {:?}", prop),
			}
		}

		channel = irc.to_string().split("#").nth(1).unwrap_or("").to_string();

		ChannelState {
			r9k_mode: r9k,
			sub_only: sub,
			slow_mode: slow,
			emote_only: emote,
			channel: channel,
			broadcaster_lang: lang,
		}
	}
}

impl Clone for ChannelState {
	fn clone(&self) -> ChannelState {

		ChannelState {
			r9k_mode: self.r9k_mode.clone(),
			sub_only: self.sub_only.clone(),
			slow_mode: self.slow_mode.clone(),
			emote_only: self.emote_only.clone(),

			channel: self.channel.clone(),
			broadcaster_lang: self.broadcaster_lang.clone(),
		}
	}
}

fn bool_from_nth_str(split: &mut Split<&str>, nth: usize) -> bool {
	split.nth(nth)
		.unwrap_or("false")
		.parse::<bool>()
		.unwrap_or(false)
}

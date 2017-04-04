/*
Tristen Horton
tristen@tristenhorton.com
2017-04-03

Module for basic channel data
Its possible to be in multiple channels at once in Twitch, so we want a simple way
to represent multiple channels at once - we'll be using this module and structure
for that.
*/

use message::Message;
use std::str::Split;

pub struct Channel {
	name: String,
	state: ChannelState
}

impl Channel {
	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn state(&self) -> &ChannelState {
		&self.state
	}
}

pub struct ChannelState {

	r9k_mode: bool,
	sub_only: bool,
	slow_mode: bool,
	emote_only: bool,

	broadcaster_lang: String,
}

impl ChannelState {
	pub fn from(irc: String) -> ChannelState {
		let properties = irc.split_whitespace().nth(0);
		let mut r9k: bool = false;
		let mut sub: bool = false;
		let mut slow: bool = false;
		let mut emote: bool = false;
		let mut lang: String = String::from("");
		let mut channel: String = String::from("");

		macro_rules! parse_from_nth {
			($tag:ident, $e:expr) => ($tag.nth($e).unwrap().parse().unwrap());
		}

		for prop in properties.unwrap().split(";") {
			let mut prop_split = prop.split("=");
			match prop_split.nth(0).unwrap() {

				"broadcaster-lang" => lang = prop_split.nth(1).unwrap().to_string(),

				"emote-only" => emote = bool_from_nth_str(&mut prop_split, 1),
				"r9k" => r9k = bool_from_nth_str(&mut prop_split, 1),
				"slow" => slow = bool_from_nth_str(&mut prop_split, 1),
				"subs-only" => sub = bool_from_nth_str(&mut prop_split, 1),

				_ => println!("[twitch-rs] Unnacounted for irc property: {:?}", prop),
			}
		}

		channel = irc.to_string().split("#").nth(1).unwrap().to_string();

		ChannelState {
			r9k_mode: r9k,
			sub_only: sub,
			slow_mode: slow,
			emote_only: emote,
			broadcaster_lang: lang,
		}
	}
}

fn bool_from_nth_str(split: &mut Split<&str>, nth: usize) -> bool {
	split.nth(nth)
		.unwrap_or("false")
		.parse::<bool>()
		.unwrap_or(false)
}

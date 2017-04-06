
use user::{UserType, UserState;
use std::collections::HashMap;

pub struct ChatMessage {

	// TODO: add emote and color support


	bot_username: String,
	user_id: String,
	username: String,
	display_name: String,
	hex_color: String,
	// TODO: represent color as a type?
	text: String,
	user_type: UserType,
	channel: String,
	is_subscriber: bool,
	is_turbo: bool,
	is_moderator: bool,
	is_me: bool,
	is_broadcaster: bool,
	raw_irc: String,
	no_emote_text: Option<String>,
	badges: HashMap<String, String>,
	// TODO: cheer badge
	bits: i32,
	bits_usd: f32,
	user: User,
}

// non-static functions
impl ChatMessage {

	pub fn text(&self) -> &String {
		&self.text
	}

	pub fn user(&self) -> &User {
		&self.user
	}
}

// static functions
impl ChatMessage {

	pub fn from(bot_username: &String, irc: &String) -> ChatMessage {

		let mut user_type = UserType::Viewer;
		let mut parsed_channel = String::new();
		let mut parsed_name = String::new();
		let mut badge_map = HashMap::<String, String>::new();
		let mut bits = 0;
		let mut usd: f32 = 0;
		let mut 

		let parts = irc.split(";");
		for part in parts {
			if part.contains("!") {
				if parsed_channel.is_empty() {
					parsed_channel = part.split("#").nth(1).unwrap().split(" ").nth(0).unwrap();
				}
				if parsed_name.is_empty() {
					parsed_name = part.split("!").nth(1).unwrap().split("@").nth(0).unwrap();
				}
				let eq_split = part.split("=");
				match eq_split.nth(1) {
					Some(item) =>  {
						if eq_split.nth(1).unwrap().contains(" ") {
							match eq_split.nth(1).unwrap().split(" ").nth(0).unwrap() {
								"mod" => user_type = UserType::Moderator;
								"global_mod" => user_type = UserType::GlobalModerator;
								"admin" => user_type = UserType::Admin;
								"staff" => user_type = UserType::Staff;
								_ => user_type = UserType::Viewer;
							}
						}
					},
					_ => {}
				}
			}
			else if part.contains("@badges=") {
				let badges = part.split("=").nth(1).unwrap();
				if badges.contains("/") {
					if badges.contains(",") {
						for badge in badges.split(",") {
							badge_map.insert(
								badge.split("/").nth(0).unwrap(),
								badge.split("/").nth(1).unwrap()
							);
						}
					}
					else {
						badge_map.insert(
							badges.split("/").nth(0).unwrap(),
							badges.split("/").nth(1).unwrap()
						);
					}
				}
			}
			else if part.contains("bits=") {
				bits = part.split("=").nth(1).unwrap().parse::<i32>().unwrap();
				usd = bits_to_usd(&bits);
			}
			// TODO: check for color, display-name, emotes, subscriber, turbo, user-id, and mod
		}
		// TODO: continue parsing irc message, see TwitchClient.cs:161
	}

	pub fn to_bool(data: &String) -> bool {
		data == String::from("1")
	}

	pub fn bits_to_usd(bits: &i32) -> f32 {
		/*
		conversion rates:
		100 bits = $1.40
		500 bits = $7.00
		1500 bits = $19.95 (5%)
		5000 bits = $64.40 (8%)
		10000 bits = $126.00 (10%)
		25000 bits = $308.00 (12%)
		*/

		return if bits < 1500 {
			(bits / 100) * 1.4
		}
		else if bits < 5000 {
			(bits / 1500) * 19.95
		}
		else if bits < 10000 {
			(bits / 5000) * 64.40
		}
		else if bits < 25000 {
			(bits / 10000) * 126
		}
		else {
			(bits / 25000) * 308
		};
	}
}


use user::{UserType, UserState};
use std::collections::HashMap;
use std::ops::Div;

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
	//user: User,
}

// non-static functions
impl ChatMessage {

	pub fn bot_username(&self) -> &String {
		&self.bot_username
	}

	pub fn user_id(&self) -> &String {
		&self.user_id
	}

	pub fn username(&self) -> &String {
		&self.username
	}

	pub fn display_name(&self) -> &String {
		&self.display_name
	}

	pub fn hex_color(&self) -> &String {
		&self.hex_color
	}

	pub fn text(&self) -> &String {
		&self.text
	}

	pub fn user_type(&self) -> &UserType {
		&self.user_type
	}

	pub fn channel(&self) -> &String {
		&self.channel
	}

	pub fn is_subscriber(&self) -> &bool {
		&self.is_subscriber
	}

	pub fn is_turbo(&self) -> &bool {
		&self.is_turbo
	}

	pub fn is_moderator(&self) -> &bool {
		&self.is_moderator
	}

	pub fn is_broadcaster(&self) -> &bool {
		&self.is_broadcaster
	}

	/*pub fn user(&self) -> &User {
		&self.user
	}*/
}

// static functions
impl ChatMessage {

	pub fn from(
		bot_username: String, 
		irc: String,
		/*chan_emotes: &mut Vec<String>,
		replace_emotes: bool*/
	) -> ChatMessage {

		let mut user_type = UserType::Viewer;
		let mut parsed_channel = String::new();
		let mut parsed_name = String::new();
		let mut badge_map = HashMap::<String, String>::new();
		let mut bits = 0;
		let mut usd: f32 = 0.0;

		let parts = irc.split(";");
		for part in parts {
			if part.contains("!") {
				if parsed_channel.is_empty() {
					parsed_channel = part.split("#")
						.nth(1).unwrap().split(" ")
						.nth(0).unwrap().to_owned();
				}
				if parsed_name.is_empty() {
					parsed_name = part.split("!")
						.nth(1).unwrap().split("@")
						.nth(0).unwrap().to_owned();
				}
				let mut eq_split = part.split("=");
				match eq_split.nth(1) {
					Some(item) =>  {
						if eq_split.nth(1).unwrap().contains(" ") {
							match eq_split.nth(1).unwrap().split(" ").nth(0).unwrap() {
								"mod" => user_type = UserType::Moderator,
								"global_mod" => user_type = UserType::GlobalModerator,
								"admin" => user_type = UserType::Admin,
								"staff" => user_type = UserType::Staff,
								_ => user_type = UserType::Viewer,
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
								badge.split("/").nth(0).unwrap().to_owned(),
								badge.split("/").nth(1).unwrap().to_owned()
							);
						}
					}
					else {
						badge_map.insert(
							badges.split("/").nth(0).unwrap().to_owned(),
							badges.split("/").nth(1).unwrap().to_owned()
						);
					}
				}
			}
			else if part.contains("bits=") {
				bits = part.split("=").nth(1).unwrap().parse::<i32>().unwrap();
				usd = ChatMessage::bits_to_usd(&bits);
			}
			// TODO: check for color, display-name, emotes, subscriber, turbo, user-id, and mod
		}
		// TODO: continue parsing irc message, see TwitchClient.cs:161
		unimplemented!()
	}

	pub fn to_bool(data: &String) -> bool {
		data == &String::from("1")
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

		return if bits.lt(&1500) {
			bits.div(100) as f32 * 1.4
		}
		else if bits.lt(&5000) {
			bits.div(1500) as f32 * 19.95
		}
		else if bits.lt(&10000) {
			bits.div(5000) as f32 * 64.40
		}
		else if bits.lt(&25000) {
			bits.div(10000) as f32 * 126.0
		}
		else {
			bits.div(25000) as f32 * 308.0
		};
	}
}

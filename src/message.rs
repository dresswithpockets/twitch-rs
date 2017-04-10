
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

	pub fn bot_username(&self) -> &str {
		self.bot_username.as_str()
	}

	pub fn user_id(&self) -> &str {
		self.user_id.as_str()
	}

	pub fn username(&self) -> &str {
		self.username.as_str()
	}

	pub fn display_name(&self) -> &str {
		self.display_name.as_str()
	}

	pub fn hex_color(&self) -> &str {
		self.hex_color.as_str()
	}

	pub fn text(&self) -> &str {
		self.text.as_str()
	}

	pub fn user_type(&self) -> UserType {
		self.user_type
	}

	pub fn channel(&self) -> &str {
		self.channel.as_str()
	}

	pub fn is_subscriber(&self) -> bool {
		self.is_subscriber
	}

	pub fn is_turbo(&self) -> bool {
		self.is_turbo
	}

	pub fn is_moderator(&self) -> bool {
		self.is_moderator
	}

	pub fn is_broadcaster(&self) -> bool {
		self.is_broadcaster
	}

	/*pub fn user(&self) -> &User {
		&self.user
	}*/
}

// static functions
impl ChatMessage {

		/*chan_emotes: &mut Vec<String>,
		replace_emotes: bool*/
	pub fn from(
		bot_username: &str, 
		irc: &str,
	) -> ChatMessage {

		let mut user_type = UserType::Viewer;
		let mut user_id = "";
		let mut channel = "";
		let mut name = "";
		let mut display_name = "";
		let mut text = "";
		let mut hex_color = "";
		let mut badge_map = HashMap::<String, String>::new();
		let mut bits = 0;
		let mut usd: f32 = 0.0;

		let mut is_subscriber = false;
		let mut is_turbo = false;
		let mut is_moderator = false;
		let mut is_me = false;
		let mut is_broadcaster = false;

		let mut no_emot_text: Option<String> = None;

		let parts = irc.split(";");
		for part in parts {
			if part.contains("!") {
				if channel.is_empty() {
					channel = part.split("#")
						.nth(1).unwrap().split(" ")
						.nth(0).unwrap();
				}
				if name.is_empty() {
					name = part.split("!")
						.nth(1).unwrap().split("@")
						.nth(0).unwrap();
				}
				let mut eq_split = part.split("=");
				match eq_split.nth(1) {
					Some(item) =>  {
						if item.contains(" ") {
							match item.split(" ").nth(0).unwrap() {
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
			else if part.contains("color=") {
				hex_color = part.split("=").nth(1).unwrap_or("");
			}
			else if part.contains("display-name") {
				display_name = part.split("=").nth(1).unwrap_or("");
			}
			else if part.contains("emotes=") {
				// TODO: Store emotes?
			}
			else if part.contains("subscriber=") {
				is_subscriber = part.split("=").nth(1).unwrap_or("0") == "1";
			}

			// TODO: check for color, display-name, emotes, subscriber, turbo, user-id, and mod
		}
		// TODO: continue parsing irc message, see TwitchClient.cs:161

		if channel.to_lowercase() == name.to_lowercase() {
			user_type = UserType::Broadcaster;
		}

		unimplemented!()
	}

	pub fn to_bool(data: &str) -> bool {
		data == "1"
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

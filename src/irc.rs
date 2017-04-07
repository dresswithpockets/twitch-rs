/*
Tristen Horton
tristen@tristenhorton.com
2017-04-03 23:50
*/

use message::ChatMessage;
use channel;
use regex::Regex;

/// Determines if an irc message is a connected message
pub fn detect_connected(irc: &String) -> bool {

	match irc.split(":").nth(2) {
		Some("You are in a maze of twisty passages, all alike.") => true,
		_ => false
	}
}

/// Determines if an irc message is a new subscriber message
pub fn detect_new_subscriber(
	irc: &String,
	channels: &Vec<channel::Channel>
) -> Option<String> {

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {

				if rtype == "PRIVMSG" &&
					irc.split("!").nth(0).unwrap_or("") == ":twitchnotify" &&
					(
						irc.contains("just subscribed!") ||
						irc.to_lowercase().contains("just subscriber with twitch prime!")
					)
				{
					return Some(chan.name().clone());
				}
			}
			_ => {}
		}
	}

	None
}

/// Determines if an irc message is a message received message
pub fn detect_message_received(
	irc: &String,
	channels: &Vec<channel::Channel>
) -> Option<String> {

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {

				if rtype == "PRIVMSG" &&
					irc.split("!").nth(0).unwrap_or("") != ":twitchnotify"
				{
					return Some(chan.name().clone());
				}
			}
			_ => {}
		}
	}

	None
}

/// Determines of an irc message is a command received message
pub fn detect_command_received(
	bot_username: &String,
	irc: &String,
	channels: &Vec<channel::Channel>,
	emotes: &Vec<String>,
	replace_emotes: &bool,
	cmd_identifiers: &Vec<String>
) -> Option<String> {

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {
				if rtype == "PRIVMSG" {
					let chat_msg = ChatMessage::from(
						bot_username.clone(),
						irc.clone(),
						emotes,
						replace_emotes.clone()
					);
					if cmd_identifiers.iter()
						.map(|x| chat_msg.text().starts_with(x))
						.any(|x| x)
					{
						return Some(chan.name().clone());
					}
				}
			}
			_ => {}
		}
	}

	None
}

/// Determines if an irc message is a user joined message
pub fn detect_user_joined(
	irc: &String,
	channels: &Vec<channel::Channel>
) -> Option<String> {

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {
				if rtype == "PART" {
					return Some(chan.name().clone());
				}
			}
			_ => {}
		}
	}

	None
}

/// Determines if an irc message is a moderator joined message
pub fn detect_moderator_joined(
	irc: &String,
	channels: &Vec<channel::Channel>
) -> Option<String> {

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {
				if rtype == "MODE" &&
					irc.contains(" ") &&
					irc.split(" ").nth(3).unwrap_or("") == "+o"
				{
					return Some(chan.name().clone());
				}
			}
			_ => {}
		}
	}

	None
}

/// Determines if an irc message is an incorrect login message
pub fn detect_incorrect_login(irc: &String) -> bool {
	irc.contains(":") &&
		irc.split(":").nth(2).unwrap_or("") == "Login authentication failed"
}

/// Determines if an irc message is a malformed oauth message
pub fn detect_malformed_oauth(
	irc: &String,
	channels: &Vec<channel::Channel>
) -> Option<String> {

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {
				if rtype == "NOTICE" &&
					irc.contains("Improperly formatted auth")
				{
					return Some(chan.name().clone());
				}
			}
			_ => {}
		}
	}
	None
}

/// Determines if an irc message is a host left message
pub fn detect_host_left(
	irc: &String,
	channels: &Vec<channel::Channel>
) -> Option<String> {

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {
				if rtype == "NOTICE" &&
					irc.contains("has gone offline")
				{
					return Some(chan.name().clone());
				}
			}
			_ => {}
		}
	}
	None
}

/// Determines if an irc message is a channel state change message
pub fn detect_channel_state_changed(
	irc: &String,
	channels: &Vec<channel::Channel>
) -> Option<String> {
	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {
				if rtype == "ROOMSTATE" {
					return Some(chan.name().clone());
				}
			}
			_ => {}
		}
	}
	None
}

/// Determines if an irc message is a user state change message
pub fn detect_user_state_changed(
	irc: &String,
	channels: &Vec<channel::Channel>
) -> Option<String> {
	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {
				if rtype == "USERSTATE" {
					return Some(chan.name().clone());
				}
			}
			_ => {}
		}
	}
	None
}

/// Determines the read type of an irc message
pub fn read_type(irc: &String, channel: &String) -> Option<String> {

	if irc.contains(" ") {

		let mut found = false;
		for word in irc.split(" ") {
			if word.chars().nth(0).unwrap() == '#' {
				if word == format!("#{}", channel) {
					found = true;
				}
			}
		}
		if found {
			let reg = Regex::new(format!(" #{}", channel).as_ref()).unwrap();
			let mut reg_split = reg.split(irc).nth(0).unwrap()
									.split(" ");
			let mut reg_copy = reg_split.clone();
			return Some(reg_copy.nth(reg_split.count() - 1).unwrap().to_owned());
		}
		else {
			if irc.split(" ").count() > 1 && irc.split(" ").nth(1).unwrap() == String::from("NOTICE") {
				return Some(String::from("NOTICE"));
			}
		}
	}

	None
}

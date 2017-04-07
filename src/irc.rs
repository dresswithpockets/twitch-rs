/*
Tristen Horton
tristen@tristenhorton.com
2017-04-03 23:50
*/

use message::ChatMessage;
use channel;
use regex::Regex;

pub fn detect_connected(message: &String) -> bool {

	match message.split(":").nth(2) {
		Some("You are in a maze of twisty passages, all alike.") => true,
		_ => false
	}
}

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

pub fn detect_incorrect_login(irc: &String) -> bool {
	irc.contains(":") &&
		irc.split(":").nth(2).unwrap_or("") == "Login authentication failed"
}

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
			// TODO: Clean this up because this is messy as heck
			let reg = Regex::new(format!(" #{}", channel).as_ref()).unwrap();
			let mut split_a = reg.split(irc);
			let mut split_a_0 = split_a.nth(0).unwrap().to_owned();
			let mut type_wrap = split_a_0.split(" ");
			let mut type_copy = type_wrap.clone();
			let wrap_count = type_wrap.count();
			let ret = type_copy.nth(wrap_count - 1).unwrap().to_owned();
			return Some(ret);
		}
		else {
			if irc.split(" ").count() > 1 && irc.split(" ").nth(1).unwrap() == String::from("NOTICE") {
				return Some(String::from("NOTICE"));
			}
		}
	}

	None
}

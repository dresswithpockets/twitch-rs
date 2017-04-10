/*
Tristen Horton
tristen@tristenhorton.com
2017-04-03 23:50
*/

use message::ChatMessage;
use channel;
use regex::Regex;

/// Determines if an irc message is a connected message
pub fn irc_connected<T>(irc: &str) -> bool {
	println!("irc_connected");
	return match irc.split(":").nth(2) {
		Some("You are in a maze of twisty passages, all alike.") => true,
		_ => false
	};
}

/// Determines if an irc message is a new subscriber message
pub fn irc_new_subscriber<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {
	println!("irc_new_subscriber");

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {

				println!("privmsg");
				if rtype == "PRIVMSG" &&
					irc.split("!").nth(0).unwrap_or("") == ":twitchnotify" &&
					(
						irc.contains("just subscribed!") ||
						irc.to_lowercase().contains("just subscriber with twitch prime!")
					)
				{
					return Some(chan.name());
				}
			}
			_ => {}
		}
	}

	None
}

/// Determines if an irc message is a message received message
pub fn irc_message_received<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {
	println!("irc_message_received");

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {

				if rtype == "PRIVMSG" &&
					irc.split("!").nth(0).unwrap_or("") != ":twitchnotify"
				{
					println!("\treturning irc message");
					return Some(chan.name());
				}
			}
			_ => {}
		}
	}

	None
}

/// Determines if an irc message is a chat message that contains a command
pub fn irc_command_received(
	bot_username: &str,
	irc: &str,
	channel: &str,
	cmd_ident: &str
) -> bool {
	read_type(irc, channel) == Some("PRIVMSG") && 
		ChatMessage::from(bot_username, irc).text().starts_with(cmd_ident)
}

/// Determines if an irc message is a command received message
pub fn irc_command_received_long(
	bot_username: &str,
	irc: &str,
	channels: &[channel::Channel],
	cmd_identifiers: &[&str]
) -> Option<String> {
	println!("irc_command_received");

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {
				if rtype == "PRIVMSG" {
					let chat_msg = ChatMessage::from(
						bot_username.clone(),
						irc.clone()
					);
					if cmd_identifiers.iter()
						.filter(|x| chat_msg.text().starts_with(x))
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
pub fn irc_user_joined<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {
	println!("irc_user_joined");

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {
				if rtype == "JOIN" {
					return Some(chan.name().clone());
				}
			}
			_ => {}
		}
	}

	None
}

/// Determines if an irc message is a user left message
pub fn irc_user_left<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {
	println!("irc_user_left");

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
pub fn irc_moderator_joined<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {

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

/// Determines if an irc message is a moderator left message
pub fn irc_moderator_left<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {
				if rtype == "MODE" &&
					irc.contains(" ") &&
					irc.split(" ").nth(3).unwrap_or("") == "-o"
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
pub fn irc_incorrect_login<T>(irc: &str) -> bool {
	irc.contains("Login authentication failed")
}

/// Determines if an irc message is a malformed oauth message
pub fn irc_malformed_oauth<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {

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
pub fn irc_host_left<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {

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
pub fn irc_channel_state_changed<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {
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
pub fn irc_user_state_changed<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {
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

/// Determines if an irc message is a re subscriber message
pub fn irc_re_subscriber<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {
				if rtype == "USERNOTICE" &&
					(
						rtype.split(";").nth(7).unwrap_or("")
						.split("=").nth(1).unwrap_or("") == "resub"
					)
				{
					Some(chan.name().clone());
				}
			}
			_ => {}
		}
	}

	None
}

/// Determines if an irc message is a ping message
pub fn irc_ping<T>(
	irc: &str
) -> bool {

	irc == "PING :tmi.twitch.tv"
}

/// Determines if an irc message is a pong message
pub fn irc_pong<T>(
	irc: &str
) -> bool {

	irc == ":tmi.twitch.tv PONG tmi.twitch.tv :irc.chat.twitch.tv"
}

/// Determines if an irc message is a hosting stopped message
pub fn irc_hosting_stopped<T>(
	irc: &str
) -> bool {

	irc.split(" ").nth(1).unwrap_or("") == "HOSTTARGET" &&
		irc.split(" ").nth(3).unwrap_or("") == ":-"
}

/// Determines if an irc message is a hosting started message
pub fn irc_hosting_started<T>(
	irc: &str
) -> bool {

	irc.split(" ").nth(1).unwrap_or("") == "HOSTTARGET" &&
		irc.split(" ").nth(3).unwrap_or("") != ":-"
}

/// Determines if an irc message is an existing users message
pub fn irc_existing_users<T>(
	irc: &str,
	username: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {

	if channels.len() > 0 &&
		irc.split(" ").count() > 5 &&
		irc.split(" ").nth(0).unwrap_or("") == format!(":{}.tmi.twitch.tv", username) &&
		irc.split(" ").nth(1).unwrap_or("") == "353" &&
		irc.split(" ").nth(2).unwrap_or("") == username
	{
		return Some(channels[channels.len() - 1].name().clone());
	}

	None
}

/// Determines if an irc message is a cleared chat message
pub fn irc_cleared_chat<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {
	for chan in channels {
		if irc == &format!(":tmi.twitch.tv CLEARCHAT #{}", chan.name()) {
			return Some(chan.name());
		}
	}

	None
}

/// Determines if an irc message is a user timedout message
pub fn irc_user_timedout<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {

				if rtype == "CLEARCHAT" &&
					irc.starts_with("@ban-duration")
				{
					return Some(chan.name());
				}
			}
			_ => {}
		}
	}
	
	None
}

/// Determines if an irc message is a user banned message
pub fn irc_user_banned<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {

				if rtype == "CLEARCHAT" &&
					irc.starts_with("@ban-reason")
				{
					return Some(chan.name());
				}
			}
			_ => {}
		}
	}

	None
}

/// Determines if an irc message is a moderators received message
pub fn irc_moderators_received<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {

				if rtype == "NOTICE" &&
					irc.contains("The moderators of this room are:")
				{
					return Some(chan.name());
				}
			}
			_ => {}
		}
	}

	None
}

/// Determines if an irc message is a chat color changed message
pub fn irc_chat_color_changed<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {

	for chan in channels {
		match read_type(irc, chan.name()) {
			Some(rtype) => {

				if rtype == "NOTICE" &&
					irc.contains("Your color has been changed.")
				{
					return Some(chan.name());
				}
			}
			_ => {}
		}
	}

	None
}

/// Determines if an irc message is a now hosting message
pub fn irc_now_hosting<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {

	match msg_id(irc) {
		Some(id) => {

			for chan in channels {
				if irc.contains(":Now hosting ") &&
					(
					chan.name().to_lowercase() ==
					irc.split(" ").nth(3).unwrap_or("").to_lowercase().replace("#", "")
					)
				{
					return Some(chan.name());
				}
			}
		}
		_ => {}
	}

	None
}

/// Determines if an irc message is a join channel completed message
pub fn irc_join_channel_completed<T>(irc: &str) -> Option<T>
where T: Into<String> {

	if irc.contains(" ") && irc.split(" ").nth(1).unwrap_or("") != "366" {
		return Some(irc.split(" ").nth(3).unwrap_or("").replace("#", "").as_ref());
	}

	None
}

/// Determines if an irc message is a being hosted message
pub fn irc_being_hosted<T>(
	irc: &str,
	channels: &Vec<channel::Channel>
) -> Option<T>
where T: Into<String> {

	if irc.contains(" ") &&
		irc.split(" ").nth(1).unwrap_or("") == "PRIVMSG" &&
		irc.contains("jtv!jtv@jtv") &&
		irc.contains("is now hosting you")
	{
		return Some(irc.split(" ").nth(2).unwrap_or(""));
	}

	None
}

/// Determines the read type of an irc message
pub fn read_type(irc: &str, channel: &str) -> Option<String> {

	return if irc.contains(" ") {
		match irc.split(" ").filter(|word| word == format!("#{}", channel.trim())).next() {
			Some(word) => {
				let reg = Regex::new(format!(" #{}", channel).as_ref()).unwrap();
				let mut reg_split = reg.split(irc).nth(0).unwrap()
										.split(" ");
				let mut reg_copy = reg_split.clone();

				reg_copy.nth(reg_split.count() - 1)
			}
			None => {
				if irc.split(" ").nth(1) == Some("NOTICE") {

					Some("NOTICE")
				}
			}
		}
	}
	else {
		None
	};

	/*if irc.contains(" ") {

		let mut found = false;
		for word in irc.split(" ") {
			if word == format!("#{}", channel.trim()) {

				found = true;
				break;
			}
		}
		if found {
			let reg = Regex::new(format!(" #{}", channel).as_ref()).unwrap();
			let mut reg_split = reg.split(irc).nth(0).unwrap()
									.split(" ");
			let mut reg_copy = reg_split.clone();

			return Some(reg_copy.nth(reg_split.count() - 1).unwrap());
		}
		else {
			if irc.split(" ").count() > 1 && irc.split(" ").nth(1).unwrap() == "NOTICE" {

				return Some("NOTICE");
			}
		}
	}

	None*/
}

/// Extracts the message id from an irc message
pub fn msg_id(irc: &str) -> Option<&str> {
	for part in irc.split(" ") {
		if part.contains("@msg-id") &&
			part.split("=").nth(1).unwrap_or("") != ""
		{
			return Some(part.split("=").nth(1).unwrap_or(""));
		}
	}
	None
}
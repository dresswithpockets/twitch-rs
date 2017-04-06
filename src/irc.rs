/*
Tristen Horton
tristen@tristenhorton.com
2017-04-03 23:50
*/

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

				if irc.split("!").nth(0).unwrap() == ":twitchnotify" &&
					rtype == "PRIVMSG" &&
					(irc.contains("just subscribed!") ||
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

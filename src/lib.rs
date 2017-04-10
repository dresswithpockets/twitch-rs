
extern crate ws;
extern crate url;
extern crate regex;
extern crate time;

pub mod message;
pub mod user;
pub mod client;
pub mod kraken;
pub mod irc;
pub mod channel;
pub mod rfc;
pub mod subscriber;
mod util;

use client::{TwitchClient, Event};

macro_rules! get_arg {
	($e:expr) => (env::args().nth($e).unwrap_or(String::from("")));
}

#[cfg(test)]
mod tests {

	use std::env;
	use client::TwitchClient;
	use client::Event;

	#[test]
	fn it_works() {

		//xn3lxlkbmjs7f6pj1gquvtfgnnzn08
		let mut user = String::from("phxvyper");
		let mut pass = String::from("oauth:kt3w160ut7p03og2no2s1zowyef2eo");
		let mut channel = String::from("phxvyper");
		let mut cmd = Vec::<String>::new();
		cmd.push(String::from("!"));
		let mut logging = true;

		println!("got here");
		/*for (index, arg) in env::args().enumerate() {
			println!("not here");
			if index == 0 { continue; }

			match arg.as_ref() {
				"-u" => user = get_arg!(index + 1),
				"-p" => pass = get_arg!(index + 1),
				"-c" => channel = get_arg!(index + 1),
				"-l" => logging = true,
				"-i" => cmd.push(get_arg!(index + 1)),
				_ => panic!("Unexpected argument passed: {}", arg),
			}
		}*/

		match TwitchClient::connect(user, pass, channel, logging, cmd, on_event) {
			Ok(()) => {
				println!("Connection fine");
			}
			_ => {
				println!("It looks like there was an error");
			}
		}

		//await
		loop {}
	}

	fn on_event(client: &TwitchClient, event: Event) {
		match event {
			Event::IncorrectLogin(message) => {
				println!("{}", message);
			},
			Event::Disconnected(user, message) => {
				println!("{}: {}", user, message);
			},
			Event::MessageReceived(message) => {
				println!("{}: {}", message.display_name(), message.text());
			}
			_ => {}
		}
	}
}

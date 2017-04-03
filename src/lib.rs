extern crate hyper;

mod message;
mod user;
mod client;

use std::env;
use client::TwitchClient;

macro_rules! get_arg {
	($e:expr) => (env::args().nth($e).unwrap_or(String::from("")));
}

#[cfg(test)]
mod tests {

	#[test]
	fn it_works() {

		let client: TwitchClient;
		let user: String;
		let pass: String;
		let channel: String;
		let logging: bool;

		for (index, arg) in env::args().iter().enumerate() {
			if index == 0 { continue; }

			match arg {
				"-u" => user = get_arg!(index + 1),
				"-p" => pass = get_arg!(index + 1),
				"-c" => channel = get_arg!(index + 1),
				"-l" => logging = true,
			}
		}

		// TODO: check to see if user and pass are matched, if not, panic

		client = TwitchClient {
			user: user,
			auth: pass,
			default_channel: channel,
			logging: logging
		};
	}
}

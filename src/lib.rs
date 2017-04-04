
extern crate hyper;
extern crate ws;

mod message;
mod user;
mod client;
mod kraken;
mod irc;
mod channel;

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

		let client: TwitchClient;
		let mut user = String::from("");
		let mut pass = String::from("");
		let mut channel = String::from("");
		let mut logging = false;

		for (index, arg) in env::args().enumerate() {
			if index == 0 { continue; }

			match arg.as_ref() {
				"-u" => user = get_arg!(index + 1),
				"-p" => pass = get_arg!(index + 1),
				"-c" => channel = get_arg!(index + 1),
				"-l" => logging = true,
				_ => panic!("Unexpected argument passed: {}", arg),
			}
		}

		client = TwitchClient::from(user, pass, channel, logging);

		client.connect();

		loop {
			match client.recv_event() {
				Ok(Event::MessageReceived(m)) => {

				}
				Err(err) => println!("Error {:?}", err),
			}
		}
	}
}

/*
Tristen Horton
tristen@tristenhorton.com
2017-04-06
*/

pub struct NewSubscriber {
	channel: String,
	name: String,
	twitch_prime: bool
}

impl NewSubscriber {
	pub fn from(irc: &str) -> NewSubscriber {
		let channel =
			irc.split("#").nth(1).unwrap_or("")
			.split(" ").nth(0).unwrap_or("")
			.replace(" ", "");

		let mut name =
			irc.split(":").nth(2).unwrap_or("")
			.split(" ").nth(0).unwrap_or("")
			.replace(" ", "");

		if name.contains(":") {
			name = name.split(":").nth(0).unwrap_or("").to_string();
		}
		let prime = irc.contains("subscribed with Twitch Prime");
		
		NewSubscriber {
			channel: channel,
			name: name,
			twitch_prime: prime
		}
	}

	pub fn channel(&self) -> &str {
		self.channel.as_str()
	}

	pub fn name(&self) -> &str {
		self.name.as_str()
	}

	pub fn twitch_prime(&self) -> bool {
		self.twitch_prime
	}
}

pub struct ReSubscriber {

}

impl<'a> From<&'a str> for ReSubscriber {
	fn from(irc: &'a str) -> ReSubscriber {
		unimplemented!()
	}
}

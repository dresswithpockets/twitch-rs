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
	pub fn from(irc: String) -> NewSubscriber {
		let channel = irc.split("#").nth(1).unwrap().split(" ").nth(0).unwrap().replace(" ", "");
		let mut name = irc.split(":").nth(2).unwrap().split(" ").nth(0).unwrap().replace(" ", "");
		if name.contains(":") {
			name = name.split(":").nth(0).unwrap().to_owned();
		}
		let prime = irc.contains("subscribed with Twitch Prime");
		NewSubscriber {
			channel: channel,
			name: name,
			twitch_prime: prime
		}
	}

	pub fn channel(&self) -> &String {
		&self.channel
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn twitch_prime(&self) -> &bool {
		&self.twitch_prime
	}
}

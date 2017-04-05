pub fn valid_nick(nick: &String) -> bool {

	// TODO: Validate nick name

	false
}

pub fn pass(pass: &String) -> String {
	format!("PASS {}", pass)
}

pub fn nick(nick: &String) -> String {
	format!("NICK {}", nick)
}

pub fn user(user: &String, mode: &i32, realname: &String) -> String {
	format!("USER {} {} * :{}", user, mode.to_string(), realname)
}

pub fn oper(name: &String, pass: &String) -> String {
	format!("OPER {} {}", name, pass)
}

pub fn privmsg(dest: &String, message: &String) -> String {
	format!("PRIVMSG {} :{}", dest, message)
}

pub fn notice(dest: &String, message: &String) -> String {
	format!("NOTICE {} :{}", dest, message)
}

pub fn join(channel: &String) -> String {
	format!("JOIN {}", channel)
}

pub fn join_multi(channels: &[String]) -> String {
	format!("JOIN {}", channels.to_vec().join(","))
}

pub fn join_key(channel: &String, key: &String) -> String {
	format!("JOIN {} {}", channel, key)
}

pub fn join_key_multi(channels: &[String], keys: &[String]) -> String {
	format!("JOIN {} {}", channels.to_vec().join(","), keys.to_vec().join(","))
}

pub fn part(channel: &String) -> String {
	format!("PART {}", channel)
}

pub fn part_multi(channels: &[String]) -> String {
	format!("PART {}", channels.to_vec().join(","))
}

pub fn part_msg(channel: &String, message: &String) -> String {
	format!("PART {} :{}", channel, message)
}

pub fn part_msg_multi(channels: &[String], message: &String) -> String {
	format!("PART {} :{}", channels.to_vec().join(","), message)
}

pub fn kick(channel: &String, nick: &String) -> String {
	format!("KICK {} {}", channel, nick)
}

pub fn kick_comment(channel: &String, nick: &String, comment: &String) -> String {
	format!("KICK {} {} :{}", channel, nick, comment)
}

pub fn kick_multi(channels: &[String], nick: &String) -> String {
	format!("KICK {} {}", channels.to_vec().join(","), nick)
}

pub fn kick_comment_multi(channels: &[String], nick: &String, comment: &String) -> String {
	format!("KICK {} {} :{}", channels.to_vec().join(","), nick, comment)
}

pub fn kick_multi_nick(channel: &String, nicks: &[String]) -> String {
	format!("KICK {} {}", channel, nicks.to_vec().join(","))
}

pub fn kick_comment_multi_nick(channel: &String, nicks: &[String], comment: &String) -> String {
	format!("KICK {} {} :{}", channel, nicks.to_vec().join(","), comment)
}

pub fn kick_mutlti_nick_channel(channels: &[String], nicks: &[String]) -> String {
	format!("KICK {} {}", channels.to_vec().join(","), nicks.to_vec().join(","))
}

pub fn kick_comment_multi_nick_channel(
	channels: &[String],
	nicks: &[String],
	comment: &String
) -> String {

	format!("KICK {} {} :{}", channels.to_vec().join(","), nicks.to_vec().join(","), comment)
}

pub fn motd() -> String {
	String::from("MOTD")
}

pub fn motd_target(target: &String) -> String {
	format!("MOTD {}", target)
}

pub fn lusers() -> String {
	String::from("LUSERS")
}

pub fn lusers_mask(mask: &String) -> String {
	format!("LUSER {}", mask)
}

pub fn lusers_target(mask: &String, target: &String) -> String {
	format!("LUSER {} {}", mask, target)
}

pub fn version() -> String {
	String::from("VERSION")
}

pub fn version_target(target: &String) -> String {
	format!("VERSION {}", target)
}

pub fn stats() -> String {
	String::from("STATS")
}

pub fn stats_query(query: &String) -> String {
	format!("STATS {}", query)
}

pub fn stats_target(query: &String, target: &String) -> String {
	format!("STATS {} {}", query, target)
}

pub fn links() -> String {
	String::from("LINKS")
}

pub fn links_mask(mask: &String) -> String {
	format!("LINKS {}", mask)
}

pub fn links_remote(remote: &String, mask: &String) -> String {
	format!("LINKS {} {}", remote, mask)
}

pub fn time() -> String {
	String::from("TIME")
}

pub fn time_target(target: &String) -> String {
	format!("TIME {}", target)
}

pub fn connect(server: &String, port: &String) -> String {
	format!("CONNECT {} {}", server, port)
}

pub fn connect_remote(server: &String, port: &String, remote: &String) -> String {
	format!("CONNECT {} {} {}", server, port, remote)
}

pub fn trace() -> String {
	String::from("TRACE")
}

pub fn trace_target(target: &String) -> String {
	format!("TRACE {}", target)
}

pub fn admin() -> String {
	String::from("ADMIN")
}

pub fn admin_target(target: &String) -> String {
	format!("ADMIN {}", target)
}

pub fn info() -> String {
	String::from("INFO")
}

pub fn info_target(target: &String) -> String {
	format!("INFO {}", target)
}

pub fn serv_list() -> String {
	String::from("SERVLIST")
}

pub fn serv_list_mask(mask: &String) -> String {
	format!("SERVLIST {}", mask)
}

pub fn serv_list_mask_type(mask: &String, masktype: &String) -> String {
	format!("SERVLIST {} {}", mask, masktype)
}

pub fn squery(name: &String, text: &String) -> String {
	format!("SQUERY {} :{}", name, text)
}

pub fn list() -> String {
	String::from("LIST")
}

pub fn list_channel(channel: &String) -> String {
	format!("LIST {}", channel)
}

pub fn list_channels(channels: &[String]) -> String {
	format!("LIST {}", channels.to_vec().join(","))
}

pub fn list_channel_target(channel: &String, target: &String) -> String {
	format!("LIST {} {}", channel, target)
}

pub fn list_channels_target(channels: &[String], target: &String) -> String {
	format!("LIST {} {}", channels.to_vec().join(","), target)
}

pub fn names() -> String {
	String::from("NAMES")
}

pub fn names_channel(channel: &String) -> String {
	format!("NAMES {}", channel)
}

pub fn names_channels(channels: &[String]) -> String {
	format!("NAMES {}", channels.to_vec().join(","))
}

pub fn names_channel_target(channel: &String, target: &String) -> String {
	format!("NAMES {} {}", channel, target)
}

pub fn names_channels_target(channels: &[String], target: &String) -> String {
	format!("NAMES {} {}", channels.to_vec().join(","), target)
}

pub fn topic(channel: &String) -> String {
	format!("TOPIC {}", channel)
}

pub fn topic_new(channel: &String, newtopic: &String) -> String {
	format!("TOPIC {} :{}", channel, newtopic)
}

pub fn mode(target: &String) -> String {
	format!("MODE {}", target)
}

pub fn mode_new(target: &String, newmode: &String) -> String {
	format!("MODE {} {}", target, newmode)
}

pub fn mode_new_params(target: &String, modes: &[String], params: &[String]) -> String {
	// TODO: Parse params and modes

	unimplemented!();
}

pub fn service(nick: &String, distro: &String, info: &String) -> String {
	format!("SERVICE {} * {} * * :{}", nick, distro, info)
}

pub fn invite(nick: &String, channel: &String) -> String {
	format!("INVITE {} {}", nick, channel)
}

pub fn who() -> String {
	String::from("WHO")
}

pub fn who_mask(mask: &String) -> String {
	format!("WHO {}", mask)
}

pub fn who_mask_ircop(mask: &String, ircop: &bool) -> String {
	if ircop {
		format!("WHO {} o", mask)
	}
	format!("WHO {}", mask)
}

pub fn whois_mask(mask: &String) -> String {
	format!("WHOIS {}", mask)
}

pub fn whois_masks(masks: &[String]) -> String {
	format!("WHOIS {}", masks.to_vec().join(","))
}

pub fn whois_target_mask(target: &String, mask: &String) -> String {
	format!("WHOIS {} {}", target, mask)
}

pub fn whois_target_masks(target: &String, masks: &[String]) -> String {
	format!("WHOIS {} {}", target, masks.to_vec().join(","))
}

pub fn whowas_nick(nick: &String) -> String {
	format!("WHOWAS {}", nick)
}

// TODO: define all whowas signatures
/*
TODO: define all:
	kill, ping, pong, away, rehash, die, restart, summon, users, wallops, userhost,
	ison, quit, squit
*/

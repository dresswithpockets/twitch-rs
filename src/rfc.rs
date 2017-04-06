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

pub fn join_channel(channel: &String) -> String {
	format!("JOIN {}", channel)
}

pub fn join_channels(channels: &[String]) -> String {
	format!("JOIN {}", channels.to_vec().join(","))
}

pub fn join_channel_key(channel: &String, key: &String) -> String {
	format!("JOIN {} {}", channel, key)
}

pub fn join_channels_keys(channels: &[String], keys: &[String]) -> String {
	format!("JOIN {} {}", channels.to_vec().join(","), keys.to_vec().join(","))
}

pub fn part_channel(channel: &String) -> String {
	format!("PART {}", channel)
}

pub fn part_channels(channels: &[String]) -> String {
	format!("PART {}", channels.to_vec().join(","))
}

pub fn part_channel_message(channel: &String, message: &String) -> String {
	format!("PART {} :{}", channel, message)
}

pub fn part_channels_message(channels: &[String], message: &String) -> String {
	format!("PART {} :{}", channels.to_vec().join(","), message)
}

pub fn kick_channel(channel: &String, nick: &String) -> String {
	format!("KICK {} {}", channel, nick)
}

pub fn kick_channel_comment(channel: &String, nick: &String, comment: &String) -> String {
	format!("KICK {} {} :{}", channel, nick, comment)
}

pub fn kick_channels(channels: &[String], nick: &String) -> String {
	format!("KICK {} {}", channels.to_vec().join(","), nick)
}

pub fn kick_channels_nick_comment(channels: &[String], nick: &String, comment: &String) -> String {
	format!("KICK {} {} :{}", channels.to_vec().join(","), nick, comment)
}

pub fn kick_channel_nicks(channel: &String, nicks: &[String]) -> String {
	format!("KICK {} {}", channel, nicks.to_vec().join(","))
}

pub fn kick_channel_nicks_comment(channel: &String, nicks: &[String], comment: &String) -> String {
	format!("KICK {} {} :{}", channel, nicks.to_vec().join(","), comment)
}

pub fn kick_channels_nicks(channels: &[String], nicks: &[String]) -> String {
	format!("KICK {} {}", channels.to_vec().join(","), nicks.to_vec().join(","))
}

pub fn kick_channels_nicks_comment(
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
	if *ircop {
		return format!("WHO {} o", mask)
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

pub fn whowas_nicks(nicks: &[String]) -> String {
	format!("WHOWAS {}", nicks.to_vec().join(","))
}

pub fn whowas_nick_count(nick: &String, count: &String) -> String {
	format!("WHOWAS {} {}", nick, count)
}

pub fn whowas_nicks_count(nicks: &[String], count: &String) -> String {
	format!("WHOWAS {} {}", nicks.to_vec().join(","), count)
}

pub fn whowas_nick_count_target(nick: &String, count: &String, target: &String) -> String {
	format!("WHOWAS {} {} {}", nick, count, target)
}

pub fn whowas_nicks_count_target(nicks: &[String], count: &String, target: &String) -> String {
	format!("WHOWAS {} {} {}", nicks.to_vec().join(","), count, target)
}

pub fn kill(nick: &String, comment: &String) -> String {
	format!("KILL {} :{}", nick, comment)
}

pub fn ping_one(server: &String) -> String {
	format!("PING {}", server)
}

pub fn ping_two(server: &String, server2: &String) -> String {
	format!("PING {} {}", server, server2)
}

pub fn pong_one(server: &String) -> String {
	format!("PONG {}", server)
}

pub fn pong_two(server: &String, server2: &String) -> String {
	format!("PONG {} {}", server, server2)
}

pub fn error(message: &String) -> String {
	format!("ERROR :{}", message)
}

pub fn away() -> String {
	String::from("AWAY")
}

pub fn away_text(text: &String) -> String {
	format!("AWAY :{}", text)
}

pub fn rehash() -> String {
	String::from("REHASH")
}

pub fn die() -> String {
	String::from("DIE")
}

pub fn restart() -> String {
	String::from("RESTART")
}

pub fn summon_user(user: &String) -> String {
	format!("SUMMON {}", user)
}

pub fn summon_user_target(user: &String, target: &String) -> String {
	format!("SUMMON {} {}", user, target)
}

pub fn summon_user_target_channel(user: &String, target: &String, channel: &String) -> String {
	format!("SUMMON {} {} {}", user, target, channel)
}

pub fn users() -> String {
	String::from("USERS")
}

pub fn users_target(target: &String) -> String {
	format!("USERS {}", target)
}

pub fn wallops(text: &String) -> String {
	format!("WALLOPS :{}", text)
}

pub fn userhost_nick(nick: &String) -> String {
	format!("USERHOST {}", nick)
}

pub fn userhost_nicks(nicks: &[String]) -> String {
	format!("USERHOST {}", nicks.to_vec().join(","))
}

pub fn ison_nick(nick: &String) -> String {
	format!("ISON {}", nick)
}

pub fn ison_nicks(nicks: &[String]) -> String {
	format!("ISON {}", nicks.to_vec().join(","))
}

pub fn quit() -> String {
	String::from("QUIT")
}

pub fn quit_message(message: &String) -> String {
	format!("QUIT {}", message)
}

pub fn squit(server: &String, comment: &String) -> String {
	format!("SQUIT {} :{}", server, comment)
}

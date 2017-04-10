pub fn valid_nick(nick: &str) -> bool {

	// TODO: Validate nick name

	false
}

pub fn pass<'a>(pass: &str) -> &'a str {
	format!("PASS {}", pass).as_ref()
}

pub fn nick<'a>(nick: &str) -> &'a str {
	format!("NICK {}", nick).as_ref()
}

pub fn user<'a>(user: &str, mode: &i32, realname: &str) -> &'a str {
	format!("USER {} {} * :{}", user, mode.to_string(), realname).as_ref()
}

pub fn oper<'a>(name: &str, pass: &str) -> &'a str {
	format!("OPER {} {}", name, pass).as_ref()
}

pub fn privmsg<'a>(dest: &str, message: &str) -> &'a str {
	format!("PRIVMSG {} :{}", dest, message).as_ref()
}

pub fn notice<'a>(dest: &str, message: &str) -> &'a str {
	format!("NOTICE {} :{}", dest, message).as_ref()
}

pub fn join_channel<'a>(channel: &str) -> &'a str {
	format!("JOIN {}", channel).as_ref()
}

pub fn join_channels<'a>(channels: &[String]) -> &'a str {
	format!("JOIN {}", channels.to_vec().join(",")).as_ref()
}

pub fn join_channel_key<'a>(channel: &str, key: &str) -> &'a str {
	format!("JOIN {} {}", channel, key).as_ref()
}

pub fn join_channels_keys<'a>(channels: &[String], keys: &[String]) -> &'a str {
	format!("JOIN {} {}", channels.to_vec().join(","), keys.to_vec().join(",")).as_ref()
}

pub fn part_channel<'a>(channel: &str) -> &'a str {
	format!("PART {}", channel).as_ref()
}

pub fn part_channels<'a>(channels: &[String]) -> &'a str {
	format!("PART {}", channels.to_vec().join(",")).as_ref()
}

pub fn part_channel_message<'a>(channel: &str, message: &str) -> &'a str {
	format!("PART {} :{}", channel, message).as_ref()
}

pub fn part_channels_message<'a>(channels: &[String], message: &str) -> &'a str {
	format!("PART {} :{}", channels.to_vec().join(","), message).as_ref()
}

pub fn kick_channel<'a>(channel: &str, nick: &str) -> &'a str {
	format!("KICK {} {}", channel, nick).as_ref()
}

pub fn kick_channel_comment<'a>(channel: &str, nick: &str, comment: &str) -> &'a str {
	format!("KICK {} {} :{}", channel, nick, comment).as_ref()
}

pub fn kick_channels<'a>(channels: &[String], nick: &str) -> &'a str {
	format!("KICK {} {}", channels.to_vec().join(","), nick).as_ref()
}

pub fn kick_channels_nick_comment<'a>(channels: &[String], nick: &str, comment: &str) -> &'a str {
	format!("KICK {} {} :{}", channels.to_vec().join(","), nick, comment).as_ref()
}

pub fn kick_channel_nicks<'a>(channel: &str, nicks: &[String]) -> &'a str {
	format!("KICK {} {}", channel, nicks.to_vec().join(",")).as_ref()
}

pub fn kick_channel_nicks_comment<'a>(channel: &str, nicks: &[String], comment: &str) -> &'a str {
	format!("KICK {} {} :{}", channel, nicks.to_vec().join(","), comment).as_ref()
}

pub fn kick_channels_nicks<'a>(channels: &[String], nicks: &[String]) -> &'a str {
	format!("KICK {} {}", channels.to_vec().join(","), nicks.to_vec().join(",")).as_ref()
}

pub fn kick_channels_nicks_comment<'a>(
	channels: &[&'a str],
	nicks: &[&'a str],
	comment: &str
) -> &'a str {

	format!("KICK {} {} :{}", channels.to_vec().join(","), nicks.to_vec().join(","), comment)
	.as_ref()
}

pub fn motd<'a>() -> &'a str {
	"MOTD"
}

pub fn motd_target<'a>(target: &str) -> &'a str {
	format!("MOTD {}", target).as_ref()
}

pub fn lusers<'a>() -> &'a str {
	"LUSERS"
}

pub fn lusers_mask<'a>(mask: &str) -> &'a str {
	format!("LUSER {}", mask).as_ref()
}

pub fn lusers_target<'a>(mask: &str, target: &str) -> &'a str {
	format!("LUSER {} {}", mask, target).as_ref()
}

pub fn version<'a>() -> &'a str {
	"VERSION"
}

pub fn version_target<'a>(target: &str) -> &'a str {
	format!("VERSION {}", target).as_ref()
}

pub fn stats<'a>() -> &'a str {
	"STATS"
}

pub fn stats_query<'a>(query: &str) -> &'a str {
	format!("STATS {}", query).as_ref()
}

pub fn stats_target<'a>(query: &str, target: &str) -> &'a str {
	format!("STATS {} {}", query, target).as_ref()
}

pub fn links<'a>() -> &'a str {
	"LINKS"
}

pub fn links_mask<'a>(mask: &str) -> &'a str {
	format!("LINKS {}", mask).as_ref()
}

pub fn links_remote<'a>(remote: &str, mask: &str) -> &'a str {
	format!("LINKS {} {}", remote, mask).as_ref()
}

pub fn time<'a>() -> &'a str {
	"TIME"
}

pub fn time_target<'a>(target: &str) -> &'a str {
	format!("TIME {}", target).as_ref()
}

pub fn connect<'a>(server: &str, port: &str) -> &'a str {
	format!("CONNECT {} {}", server, port).as_ref()
}

pub fn connect_remote<'a>(server: &str, port: &str, remote: &str) -> &'a str {
	format!("CONNECT {} {} {}", server, port, remote).as_ref()
}

pub fn trace<'a>() -> &'a str {
	"TRACE"
}

pub fn trace_target<'a>(target: &str) -> &'a str {
	format!("TRACE {}", target).as_ref()
}

pub fn admin<'a>() -> &'a str {
	"ADMIN"
}

pub fn admin_target<'a>(target: &str) -> &'a str {
	format!("ADMIN {}", target).as_ref()
}

pub fn info<'a>() -> &'a str {
	"INFO"
}

pub fn info_target<'a>(target: &str) -> &'a str {
	format!("INFO {}", target).as_ref()
}

pub fn serv_list<'a>() -> &'a str {
	"SERVLIST"
}

pub fn serv_list_mask<'a>(mask: &str) -> &'a str {
	format!("SERVLIST {}", mask).as_ref()
}

pub fn serv_list_mask_type<'a>(mask: &str, masktype: &str) -> &'a str {
	format!("SERVLIST {} {}", mask, masktype).as_ref()
}

pub fn squery<'a>(name: &str, text: &str) -> &'a str {
	format!("SQUERY {} :{}", name, text).as_ref()
}

pub fn list<'a>() -> &'a str {
	"LIST"
}

pub fn list_channel<'a>(channel: &str) -> &'a str {
	format!("LIST {}", channel).as_ref()
}

pub fn list_channels<'a>(channels: &[String]) -> &'a str {
	format!("LIST {}", channels.to_vec().join(",")).as_ref()
}

pub fn list_channel_target<'a>(channel: &str, target: &str) -> &'a str {
	format!("LIST {} {}", channel, target).as_ref()
}

pub fn list_channels_target<'a>(channels: &[String], target: &str) -> &'a str {
	format!("LIST {} {}", channels.to_vec().join(","), target).as_ref()
}

pub fn names<'a>() -> &'a str {
	"NAMES"
}

pub fn names_channel<'a>(channel: &str) -> &'a str {
	format!("NAMES {}", channel).as_ref()
}

pub fn names_channels<'a>(channels: &[String]) -> &'a str {
	format!("NAMES {}", channels.to_vec().join(",")).as_ref()
}

pub fn names_channel_target<'a>(channel: &str, target: &str) -> &'a str {
	format!("NAMES {} {}", channel, target).as_ref()
}

pub fn names_channels_target<'a>(channels: &[String], target: &str) -> &'a str {
	format!("NAMES {} {}", channels.to_vec().join(","), target).as_ref()
}

pub fn topic<'a>(channel: &str) -> &'a str {
	format!("TOPIC {}", channel).as_ref()
}

pub fn topic_new<'a>(channel: &str, newtopic: &str) -> &'a str {
	format!("TOPIC {} :{}", channel, newtopic).as_ref()
}

pub fn mode<'a>(target: &str) -> &'a str {
	format!("MODE {}", target).as_ref()
}

pub fn mode_new<'a>(target: &str, newmode: &str) -> &'a str {
	format!("MODE {} {}", target, newmode).as_ref()
}

pub fn mode_new_params<'a>(target: &str, modes: &[String], params: &[String]) -> &'a str {
	// TODO: Parse params and modes

	unimplemented!();
}

pub fn service<'a>(nick: &str, distro: &str, info: &str) -> &'a str {
	format!("SERVICE {} * {} * * :{}", nick, distro, info).as_ref()
}

pub fn invite<'a>(nick: &str, channel: &str) -> &'a str {
	format!("INVITE {} {}", nick, channel).as_ref()
}

pub fn who<'a>() -> &'a str {
	"WHO"
}

pub fn who_mask<'a>(mask: &str) -> &'a str {
	format!("WHO {}", mask).as_ref()
}

pub fn who_mask_ircop<'a>(mask: &str, ircop: &bool) -> &'a str {
	if *ircop {
		return format!("WHO {} o", mask).as_ref()
	}
	format!("WHO {}", mask).as_ref()
}

pub fn whois_mask<'a>(mask: &str) -> &'a str {
	format!("WHOIS {}", mask).as_ref()
}

pub fn whois_masks<'a>(masks: &[String]) -> &'a str {
	format!("WHOIS {}", masks.to_vec().join(",")).as_ref()
}

pub fn whois_target_mask<'a>(target: &str, mask: &str) -> &'a str {
	format!("WHOIS {} {}", target, mask).as_ref()
}

pub fn whois_target_masks<'a>(target: &str, masks: &[String]) -> &'a str {
	format!("WHOIS {} {}", target, masks.to_vec().join(",")).as_ref()
}

pub fn whowas_nick<'a>(nick: &str) -> &'a str {
	format!("WHOWAS {}", nick).as_ref()
}

pub fn whowas_nicks<'a>(nicks: &[String]) -> &'a str {
	format!("WHOWAS {}", nicks.to_vec().join(",")).as_ref()
}

pub fn whowas_nick_count<'a>(nick: &str, count: &str) -> &'a str {
	format!("WHOWAS {} {}", nick, count).as_ref()
}

pub fn whowas_nicks_count<'a>(nicks: &[String], count: &str) -> &'a str {
	format!("WHOWAS {} {}", nicks.to_vec().join(","), count).as_ref()
}

pub fn whowas_nick_count_target<'a>(nick: &str, count: &str, target: &str) -> &'a str {
	format!("WHOWAS {} {} {}", nick, count, target).as_ref()
}

pub fn whowas_nicks_count_target<'a>(nicks: &[String], count: &str, target: &str) -> &'a str {
	format!("WHOWAS {} {} {}", nicks.to_vec().join(","), count, target).as_ref()
}

pub fn kill<'a>(nick: &str, comment: &str) -> &'a str {
	format!("KILL {} :{}", nick, comment).as_ref()
}

pub fn ping_one<'a>(server: &str) -> &'a str {
	format!("PING {}", server).as_ref()
}

pub fn ping_two<'a>(server: &str, server2: &str) -> &'a str {
	format!("PING {} {}", server, server2).as_ref()
}

pub fn pong_one<'a>(server: &str) -> &'a str {
	format!("PONG {}", server).as_ref()
}

pub fn pong_two<'a>(server: &str, server2: &str) -> &'a str {
	format!("PONG {} {}", server, server2).as_ref()
}

pub fn error<'a>(message: &str) -> &'a str {
	format!("ERROR :{}", message).as_ref()
}

pub fn away<'a>() -> &'a str {
	"AWAY"
}

pub fn away_text<'a>(text: &str) -> &'a str {
	format!("AWAY :{}", text).as_ref()
}

pub fn rehash<'a>() -> &'a str {
	"REHASH"
}

pub fn die<'a>() -> &'a str {
	"DIE"
}

pub fn restart<'a>() -> &'a str {
	"RESTART"
}

pub fn summon_user<'a>(user: &str) -> &'a str {
	format!("SUMMON {}", user).as_ref()
}

pub fn summon_user_target<'a>(user: &str, target: &str) -> &'a str {
	format!("SUMMON {} {}", user, target).as_ref()
}

pub fn summon_user_target_channel<'a>(user: &str, target: &str, channel: &str) -> &'a str {
	format!("SUMMON {} {} {}", user, target, channel).as_ref()
}

pub fn users<'a>() -> &'a str {
	"USERS"
}

pub fn users_target<'a>(target: &str) -> &'a str {
	format!("USERS {}", target).as_ref()
}

pub fn wallops<'a>(text: &str) -> &'a str {
	format!("WALLOPS :{}", text).as_ref()
}

pub fn userhost_nick<'a>(nick: &str) -> &'a str {
	format!("USERHOST {}", nick).as_ref()
}

pub fn userhost_nicks<'a>(nicks: &[String]) -> &'a str {
	format!("USERHOST {}", nicks.to_vec().join(",")).as_ref()
}

pub fn ison_nick<'a>(nick: &str) -> &'a str {
	format!("ISON {}", nick).as_ref()
}

pub fn ison_nicks<'a>(nicks: &[String]) -> &'a str {
	format!("ISON {}", nicks.to_vec().join(",")).as_ref()
}

pub fn quit<'a>() -> &'a str {
	"QUIT"
}

pub fn quit_message<'a>(message: &str) -> &'a str {
	format!("QUIT {}", message).as_ref()
}

pub fn squit<'a>(server: &str, comment: &str) -> &'a str {
	format!("SQUIT {} :{}", server, comment).as_ref()
}

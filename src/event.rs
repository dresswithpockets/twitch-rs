use time::Tm;

use user;
use channel;
use message;
use rfc;
use irc;
use subscriber;
use util;

/// The `Event` type. Represents a twitch chat event
pub enum Event {
	/// Invalid Event
	None,

	/// username, data, datetime
	OnLog(&str, &str, Tm),
	
	/// username, default_channel
	Connected(&str, &str),
	
	/// username, channel
	ChannelJoined(&str, &str),
	
	/// error_message
	IncorrectLogin(&str),
	
	/// channel_state, channel
	ChannelStateChanged(channel::ChannelState, &str),
	
	/// user_state
	UserStateChanged(user::UserState),
	
	/// received_message
	MessageReceived(message::ChatMessage),
	
	/// sent_message
	MessageSent(message::ChatMessage),

	/// receive_message
	WhisperSent(&str, &str),
	
	/// command_message, command, args, args_as_string, command_prefix
	ChatCommandReceived(
		message::ChatMessage,
		&str,
		&[&str],
		&str,
		&str,
	),

	WhisperCommandReceived(),

	/// username, channel
	UserJoined(&str, &str),

	/// username, channel
	ModeratorJoined(&str, &str),

	/// username, channel
	ModeratorLeft(&str, &str),

	/// channel, new_sub
	NewSubscriber(&str, subscriber::NewSubscriber),
	
	/// channel, re_sub
	ReSubscriber(&str, subscriber::ReSubscriber),

	/// username
	HostLeft(Option<&str>),

	/// channel, users
	ExistingUsersDetected(&str, &[&str]),

	/// username, channel
	UserLeft(&str, &str),

	// viewcount, hoster, hostee
	HostingStarted(i32, &str, &str),

	// viewcount, hoster
	HostingStopped(i32, &str),

	/// username, reason
	Disconnected(&str, &str),

	// username, message
	ConnectionError(&str, &str),

	/// channel
	ChatCleared(&str),
	UserTimedout(),
	LeftChannel(),
	UserBanned(),
	ModeratorsReceived(),
	ChatColorChanged(),

	/// direction, data
	SendReceiveData(SendReceiveDirection, &str),

	// channel, hostee
	NowHosting(&str, &str),

	/// bot_username, host_channel, viewers, channel_being_hosted
	BeingHosted(&str, &str, i32, &str),
}

pub enum SendReceiveDirection {
	Sent,
	Received,
}

# twitch-rs
Rust wrapper for Twitch API

### Implemented API


### Planned API

The following is everything we plan on implementing in the API before officially releasing as "version 1.0".

```rs
// Events
Event::Connected(ConnectedArgs)
Event::ChannelJoined(ChannelJoinedArgs)
Event::IncorrectLogin(IncorrectLoginArgs)
Event::ChannelStateChanged(ChannelStateChangedArgs)
Event::UserStateChanged(UserStateChangedArgs)
Event::MessageReceived(MessageReceivedArgs)
Event::WhisperReceived(WhisperReceivedArgs)
Event::MessageSent(MessageSentArgs)
Event::WhisperSent(WhisperSentArgs)
Event::ChatCommandReceived(ChatCommandReceivedArgs)
Event::WhisperCommandReceived(WhisperCommandReceivedArgs)
Event::UserJoined(UserJoinedArgs)
Event::ModeratorJoined(ModeratorJoinedArgs)
Event::ModeratorLeft(ModeratorLeftArgs)
Event::NewSubscriber(NewSubscriberArgs)
Event::ReSubscriber(ReSubscriberArgs)
Event::HostLeft(HostLeftArgs)
Event::DetectedExistingUsers(ExistingUsersArgs)
Event::UserLeft(UserLeftArgs)
Event::HostingStarted(HostingStartedArgs)
Event::HostingStopped(HostingStoppedArgs)
Event::Disconnected(DisconnectedArgs)
Event::ConnectionError(ConnectionErrorArgs)
Event::ChatCleared(ChatClearedArgs)
Event::UserTimedout(UserTimedoutArgs)
Event::LeftChannel(LeftChannelArgs)
Event::UserBanned(UserBannedArgs)
Event::ModeratorsReceived(ModeratorsReceivedArgs)
Event::ChatColorChanged(ChatColorChangedArgs)
Event::SendReceiveData(SendReceiveDataArgs)
Event::NowHosting(NowHostingArgs)
Event::BeingHosted(BeingHostedArgs)

// Client actions
Client:

connect()
disconnect()
reconnect()

send_raw(message: String)

send_message(channel: JoinedChannel, message: String)
send_message(channel: String, message: String)
send_message(message: String)

send_whisper(user: String, message: String)

join_channel(channel: String)

get_joined_channel(channel: String) -> JoinedChannel

leave_channel(channel: String)
leave_channel(channel: JoinedChannel)

req_channel_mods()
req_channel_mods(channel: String)
req_channel_mods(channel: JoinedChannel)

```

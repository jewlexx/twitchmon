use anyhow::Context as _;
use colored::{Color, Colorize};
use twitchchat::{messages, AsyncRunner, Status, UserConfig};

// some helpers for the demo
fn get_env_var(key: &str) -> anyhow::Result<String> {
    std::env::var(key).with_context(|| format!("please set `{}`", key))
}

pub fn get_user_config() -> anyhow::Result<twitchchat::UserConfig> {
    let name = get_env_var("TWITCH_NAME").unwrap_or_else(|_| twitchchat::ANONYMOUS_LOGIN.0.into());
    let token =
        get_env_var("TWITCH_TOKEN").unwrap_or_else(|_| twitchchat::ANONYMOUS_LOGIN.1.into());

    // you need a `UserConfig` to connect to Twitch
    let config = UserConfig::builder()
        // the name of the associated twitch account
        .name(name)
        // and the provided OAuth token
        .token(token)
        // and enable all of the advanced message signaling from Twitch
        .enable_all_capabilities()
        .build()?;

    Ok(config)
}

// channels can be either in the form of '#museun' or 'museun'. the crate will internally add the missing #
pub fn channels_to_join() -> anyhow::Result<Vec<String>> {
    Ok(vec!["xqcow".into()])
}

pub async fn message_loop(mut runner: AsyncRunner) -> anyhow::Result<()> {
    loop {
        match runner.next_message().await? {
            // this is the parsed message -- across all channels (and notifications from Twitch)
            Status::Message(msg) => {
                handle_message(msg).await;
            }

            // you signaled a quit
            Status::Quit => {
                println!("we signaled we wanted to quit");
                break;
            }
            // the connection closed normally
            Status::Eof => {
                println!("we got a 'normal' eof");
                break;
            }
        }
    }

    Ok(())
}

// you can generally ignore the lifetime for these types.
async fn handle_message(msg: messages::Commands<'_>) {
    use messages::Commands::*;

    // All sorts of messages
    match msg {
        // This is the one users send to channels
        Privmsg(msg) => {
            let color = {
                let (r, g, b) = match &msg.color() {
                    Some(v) => (v.rgb.0, v.rgb.1, v.rgb.2),
                    None => (255, 255, 255),
                };
                Color::TrueColor { r, g, b }
            };
            let channels = channels_to_join().unwrap_or_default();
            println!(
                "{}{}: {}",
                if channels.len() > 1 {
                    format!("[{}] ", msg.channel())
                } else {
                    "".into()
                },
                msg.name().bold().color(color),
                msg.data()
            )
        }

        // This one is special, if twitch adds any new message
        // types, this will catch it until future releases of
        // this crate add them.
        Raw(_) => {}

        // These happen when you initially connect
        IrcReady(_) => {}
        Ready(_) => {}
        Cap(_) => {}

        // and a bunch of other messages you may be interested in
        ClearChat(_) => {}
        ClearMsg(_) => {}
        GlobalUserState(_) => {}
        HostTarget(_) => {}
        Join(_) => {}
        Notice(_) => {}
        Part(_) => {}
        Ping(_) => {}
        Pong(_) => {}
        Reconnect(_) => {}
        RoomState(_) => {}
        UserNotice(_) => {}
        UserState(_) => {}
        Whisper(_) => {}

        _ => {}
    }
}

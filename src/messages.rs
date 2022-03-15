use colored::{Color, Colorize};
use twitchchat::{messages, AsyncRunner, Status};

use crate::include::channels_to_join;

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
            let channels = channels_to_join();
            println!(
                "{} {}: {}",
                if channels.len() > 1 {
                    format!("[{}]", msg.channel())
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

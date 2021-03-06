use anyhow::Context as _;
use twitchchat::UserConfig;

use clap::Parser;

/// Monitor Twitch Chat
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Names of the channels chats to join
    channels: Vec<String>,

    /// Your desired login username
    #[clap(short, long)]
    username: Option<String>,

    /// The oauth token matching the given username NOTE: NOT YOUR PASSWORD
    #[clap(short, long)]
    token: Option<String>,
}

// some helpers for the demo
fn get_env_var(key: &str) -> anyhow::Result<String> {
    std::env::var(key).with_context(|| format!("please set `{}`", key))
}

pub fn get_user_config() -> anyhow::Result<UserConfig> {
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
pub fn channels_to_join() -> Vec<String> {
    let channels = Args::parse().channels;

    if channels.is_empty() {
        println!("You must provide channels to join!");
        std::process::exit(1)
    }

    channels
}

use std::process::exit;

use anyhow::Context;
use colored::Colorize;
use twitchchat::{connector, runner::AsyncRunner, UserConfig};

mod include;
mod macros;
mod messages;

use include::{channels_to_join, get_user_config};
use messages::message_loop;

const ANON_USERNAME: &str = "justinfan1234";

async fn connect(user_config: &UserConfig, channels: &[String]) -> anyhow::Result<AsyncRunner> {
    let connector = connector::async_io::Connector::twitch()?;

    print!("Connecting...");
    flush!();
    let mut runner = AsyncRunner::connect(connector, user_config).await?;
    print!("\rConnected!");
    flush!();

    let name = runner.identity.username();
    if name == ANON_USERNAME {
        println!("\nLogged in as anonymous. NOTE: This means you will have limited privileges!");
    } else {
        println!("\nLogged in as {}", name);
    }

    for channel in channels {
        println!("Attempting to join '{}'", channel);
        let _ = runner.join(channel).await?;
        println!("Joined '{}'!", channel);
    }

    Ok(runner)
}

fn main() -> anyhow::Result<()> {
    let user_config = get_user_config()?;
    let channels = channels_to_join();

    let executor = async_executor::Executor::new();
    futures_lite::future::block_on(executor.run(async {
        // connect and join the provided channel(s)
        let runner = connect(&user_config, &channels).await?;

        ctrlc::set_handler(move || {
            print!("\r{}", "Closing down safely...\n".bright_red());

            exit(0);
        })
        .unwrap();

        println!("Starting message loop");

        message_loop(runner).await
    }))
}

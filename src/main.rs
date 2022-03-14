use twitchchat::{connector, runner::AsyncRunner, UserConfig};

mod include;
use include::{channels_to_join, get_user_config, message_loop};

async fn connect(user_config: &UserConfig, channels: &[String]) -> anyhow::Result<AsyncRunner> {
    let connector = connector::async_io::Connector::twitch()?;

    println!("we're connecting!");
    let mut runner = AsyncRunner::connect(connector, user_config).await?;
    println!("..and we're connected");

    println!("our identity: {:#?}", runner.identity);

    for channel in channels {
        println!("attempting to join '{}'", channel);
        let _ = runner.join(channel).await?;
        println!("joined '{}'!", channel);
    }

    Ok(runner)
}

fn main() -> anyhow::Result<()> {
    let user_config = get_user_config()?;
    let channels = channels_to_join()?;

    let executor = async_executor::Executor::new();
    futures_lite::future::block_on(executor.run(async {
        // connect and join the provided channel(s)
        let runner = connect(&user_config, &channels).await?;

        let _quit_handle = runner.quit_handle();

        // spawn something off in the background that'll exit in 10 seconds
        // executor
        //     .spawn({
        //         async move {
        //             println!("in 10 seconds we'll exit");
        //             async_io::Timer::after(std::time::Duration::from_secs(10)).await;

        //             println!("sending quit signal");
        //             quit_handle.notify().await;
        //         }
        //     })
        //     .detach();

        println!("starting message loop");

        message_loop(runner).await
    }))
}

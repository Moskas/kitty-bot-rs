use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;

use std::fs;

use owoify_rs::{Owoifiable, OwoifyLevel};

#[group]
#[commands(ping, test, owo, uwu)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("*")) // set the bot's prefix to "*"
        .group(&GENERAL_GROUP);

    // Login with a bot token from my user config directory (TODO change path)
    let token: String = fs::read_to_string("/home/moskas/.config/key.txt").expect(":thinking:");
    let mut client = Client::builder(token.trim())
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}
#[command]
async fn test(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Rust!").await?;

    Ok(())
}
#[command]
async fn owo(ctx: &Context, msg: &Message) -> CommandResult {
    let message: String = (msg.content_safe(ctx).await)
        .split("*owo")
        .skip(1)
        .map(str::to_string)
        .collect();
    //let info: String = message.split('~').skip(1).map(str::to_string).collect();
    msg.reply(ctx, message.owoify(&OwoifyLevel::Owo)).await?;

    Ok(())
}
#[command]
async fn uwu(ctx: &Context, msg: &Message) -> CommandResult {
    let message: String = (msg.content_safe(ctx).await)
        .split("*uwu")
        .skip(1)
        .map(str::to_string)
        .collect();

    msg.reply(ctx, message.owoify(&OwoifyLevel::Uwu)).await?;

    Ok(())
}

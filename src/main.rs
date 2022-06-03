use rand::prelude::*;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use std::fs;
use std::time::SystemTime; // Execution time testing

use owoify_rs::{Owoifiable, OwoifyLevel};

#[group]
#[commands(ping, roll, owo, uwu)]
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
async fn roll(ctx: &Context, msg: &Message) -> CommandResult {
    let sides: i32 = (msg.content_safe(ctx).await)
        .split_whitespace()
        .skip(1)
        .map(str::to_string)
        .collect::<String>()
        .parse::<i32>()
        .unwrap();
    let roll: i32 = thread_rng().gen_range(1..=sides);
    msg.reply(ctx, format!("You have rolled: {} :game_die:", roll))
        .await?;
    Ok(())
}
#[command]
async fn owo(ctx: &Context, msg: &Message) -> CommandResult {
    let message: String = (msg.content_safe(ctx).await)
        .split("*owo")
        .skip(1)
        .map(str::to_string)
        .collect();
    println!("{}", message.len());
    //let info: String = message.split('~').skip(1).map(str::to_string).collect();
    msg.reply(ctx, message.owoify(&OwoifyLevel::Owo)).await?;

    Ok(())
}
#[command]
async fn uwu(ctx: &Context, msg: &Message) -> CommandResult {
    let start = SystemTime::now();
    let message: String = (msg.content_safe(ctx).await)
        .split("*uwu")
        .skip(1)
        .map(str::to_string)
        .collect::<String>()
        .owoify(&OwoifyLevel::Uwu);
    println!("{}", message.len());
    if message.len() >= 2000 {
        msg.reply(
            ctx,
            "Owoified message is longer than 2000 characters".owoify(&OwoifyLevel::Owo),
        )
        .await?;
    //        let splitter: Vec<&str> = Vec::new();
    } else {
        msg.reply(ctx, message).await?;
    }
    let end = SystemTime::now();
    let elapsed = end.duration_since(start);
    println!(
        "Execution time: {}ms",
        elapsed.unwrap_or_default().as_millis()
    );
    Ok(())
}

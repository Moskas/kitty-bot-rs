use rand::prelude::*;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::prelude::GatewayIntents;
use std::fs;
//use std::time::SystemTime; // Execution time testing

use owoify_rs::{Owoifiable, OwoifyLevel};

#[group]
#[commands(ping, roll, choose, owo, uwu, avatar, whoami, status)]
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
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token.trim(), intents)
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
    let sides: i32 = match (msg.content_safe(ctx))
        .split_whitespace()
        .skip(1)
        .map(str::to_string)
        .collect::<String>()
        .parse::<i32>()
    {
        Ok(number) => number,
        Err(_e) => 6,
    };
    let roll: i32 = thread_rng().gen_range(1..=sides);
    msg.reply(ctx, format!("You have rolled: {} :game_die:", roll))
        .await?;
    Ok(())
}

#[command]
async fn choose(ctx: &Context, msg: &Message) -> CommandResult {
    let options: Vec<String> = msg
        .content_safe(ctx)
        .split("*choose")
        .map(str::to_string)
        .collect::<String>()
        .split("|")
        .map(str::to_string)
        .collect();
    let random_index: usize = thread_rng().gen_range(0..options.len());
    msg.reply(ctx, format!("I choose: {}", options[random_index]))
        .await?;
    Ok(())
}

#[command]
async fn owo(ctx: &Context, msg: &Message) -> CommandResult {
    let message: String = (msg.content_safe(ctx))
        .split("*owo")
        .skip(1)
        .map(str::to_string)
        .collect();
    println!("{}", message.len());
    msg.reply(ctx, message.owoify(OwoifyLevel::Owo)).await?;

    Ok(())
}
#[command]
async fn uwu(ctx: &Context, msg: &Message) -> CommandResult {
    let message: String = (msg.content_safe(ctx))
        .split("*uwu")
        .skip(1)
        .map(str::to_string)
        .collect::<String>()
        .owoify(OwoifyLevel::Uwu);
    println!("{}", message.len());
    if message.len() >= 2000 {
        msg.reply(
            ctx,
            "Owoified message is longer than 2000 characters".owoify(OwoifyLevel::Owo),
        )
        .await?;
    //        let splitter: Vec<&str> = Vec::new();
    } else {
        msg.reply(ctx, message).await?;
    }
    Ok(())
}

#[command]
async fn avatar(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(
        ctx,
        format!("Your avatar: {}", msg.author.static_avatar_url().unwrap()),
    )
    .await?;
    Ok(())
}

#[command]
async fn whoami(ctx: &Context, msg: &Message) -> CommandResult {
    println!(
        "{} {} {:?}",
        msg.author.static_avatar_url().unwrap(),
        msg.author.id,
        msg.author.accent_colour.unwrap()
    );
    msg.reply(ctx, msg.author.to_string()).await?;
    Ok(())
}

#[command]
async fn status(ctx: &Context, msg: &Message) -> CommandResult {
    msg.delete(ctx).await?;
    Ok(())
}

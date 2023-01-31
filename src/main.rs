use std::env;

use serenity::async_trait;
use serenity::Client;
use serenity::client::Context;
use serenity::framework::StandardFramework;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::{command, group};
use serenity::model::prelude::MessageUpdateEvent;
use serenity::model::prelude::{ChannelId, GuildId, MessageId};
use serenity::model::prelude::{Message, Ready};
use serenity::prelude::{GatewayIntents, EventHandler};

const LOG_CHANNEL: ChannelId = ChannelId(550190785358856223);
const LOGGED_MESSAGES: usize = 5000;

// Event Handler
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name)
    }

    async fn message_update(&self, ctx: Context, old_msg: Option<Message>, _: Option<Message>, _: MessageUpdateEvent) {
        if let Some(message) = old_msg {
            let deletion_msg = format!("Message edited by `{}`", message.author.name);
            let _ = LOG_CHANNEL.send_message(&ctx.http, |m| m.content(deletion_msg)).await;
            let _ = LOG_CHANNEL.send_message(&ctx.http, |m| m.content(message.content)).await;
        }
    }

    async fn message_delete(&self, ctx: Context, channel_id: ChannelId, message_id: MessageId, _: Option<GuildId>) {
        if let Some(message) = ctx.cache.message(channel_id, message_id) {
            let deletion_msg = format!("Message deleted by `{}`", message.author.name);
            let _ = LOG_CHANNEL.send_message(&ctx.http, |m| m.content(deletion_msg)).await;
            let _ = LOG_CHANNEL.send_message(&ctx.http, |m| m.content(message.content)).await;
        }
    }
}

#[group]
#[commands(ping)]
struct General;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "pong!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .cache_settings(|f| f.max_messages(LOGGED_MESSAGES))
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

use std::env;

use serenity::all::{ChannelId, UserId};
use serenity::async_trait;
use serenity::builder::EditMember;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::id::GuildId;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let guild = GuildId::new(1184834475817975909);
    if let Ok(members) = guild.members(&ctx.http, None, None).await {
            // if let Some(dm) =UserId::new().create_dm_channel(&ctx.http).await.ok(){
            //     if let Err(why) = dm.say(&ctx.http, "Congratulations! you have 1 🔥").await {
            //         println!("Error sending DM: {:?}", why);
            //     } else {
            //         println!("DM sent successfully!");
            //     }
            // }
            for member in members {
            let nickname =format!("{} {}" , member.user.name," | 🔥1 ") ;
            let builder = EditMember::new().nickname(nickname);
            if let Err(why) = guild.edit_member(&ctx.http, member.user.id, builder).await{
                println!("Failed to edit member: {:?}", why);
            }else{
                let chan =  ChannelId::new(1204114543488405525);
                chan.say(&ctx.http, format!("{} has 1 🔥!", member.user.name)).await.expect("Error Sending Message To channel!");
            }
        }
    } else {
        println!("Error fetching members");
    }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
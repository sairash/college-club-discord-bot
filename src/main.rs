use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serenity::all::RoleId;
use shuttle_secrets::SecretStore;
use tokio::task;
use tokio::time;
use warp::Filter;
use warp::Reply;

use serenity::all::{ChannelId, UserId};
use serenity::async_trait;
use serenity::builder::EditMember;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;

struct Handler;

#[derive(Deserialize, Serialize)]
struct MessageAtWelcome {
    author: String,
    author_id: String,
    content: String,
    content_id: Option<String>,
}

struct SharedArray {
    array: Arc<Mutex<Vec<MessageAtWelcome>>>,
}

impl SharedArray {
    fn new() -> Self {
        SharedArray {
            array: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn add_element(&self, element: MessageAtWelcome) {
        self.array.lock().unwrap().push(element);
    }

    async fn pop(&self) -> Option<MessageAtWelcome> {
        return self.array.lock().unwrap().pop();
    }

    fn count(&self) -> usize {
        return self.array.lock().unwrap().len();
    }
}

static SHARED_ARRAY: Lazy<SharedArray> = Lazy::new(|| SharedArray::new());

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
        loop {
            loop {
                if let Some(message) = SHARED_ARRAY.pop().await {
                    let guild = GuildId::new(1184834475817975909);
                    if message.content.contains("/name") {
                        let second_part = message.content.split("/name ").nth(1);
                        if let Some(nickname) = second_part {
                            let builder = EditMember::new().nickname(nickname);
                            if let Ok(author_id) = message.author_id.parse::<u64>() {
                                if let Err(why) =
                                    guild.edit_member(&ctx.http, author_id, builder).await
                                {
                                    println!("Failed to edit member: {:?}", why);
                                } else {
                                    let role_id = RoleId::new(1203279581361209354);
                                    let user_id = UserId::new(author_id);
                                    if let Err(why) = ctx
                                        .http
                                        .add_member_role(
                                            guild,
                                            user_id,
                                            role_id,
                                            Some("Reason for adding the role"),
                                        )
                                        .await
                                    {
                                        println!("Failed to add role to member: {:?}", why);
                                    }
                                    let chan = ChannelId::new(1184834476279337045);
                                    chan.say(
                                        &ctx.http,
                                        format!("{} Welcome to Web & App Dev Club!", nickname),
                                    )
                                    .await
                                    .expect("Error Sending Message To channel!");
                                }
                            }
                        } else {
                            let chan = ChannelId::new(1184834476279337045);
                            chan.say(&ctx.http, "You need to specify your name too!")
                                .await
                                .expect("Error Sending Message To channel!");
                        }
                    }
                } else {
                    break;
                }
            }
            time::sleep(Duration::from_secs(5)).await;
        }
    }
}

async fn serenity(secret_store: SecretStore) {
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return;
    };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

#[shuttle_runtime::main]
async fn warp(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    task::spawn(serenity(secret_store));

    let static_files = warp::fs::dir("frontend/dist");
    let assets_files = warp::fs::dir("frontend/dist/assets");

    let route_post = warp::post()
        .and(warp::path("message"))
        .and(warp::path::param::<String>())
        .and(warp::body::json())
        .and_then(|content_id, mut message: MessageAtWelcome| async move {
            message.content_id = Some(content_id);
            println!("Count Start {:?}", SHARED_ARRAY.count());
            SHARED_ARRAY.add_element(message);
            println!("Count End {:?}", SHARED_ARRAY.count());
            Ok::<_, warp::Rejection>(warp::reply::html("Sent"))
        })
        .or(warp::path::end().and(static_files))
        .or(warp::path("assets").and(assets_files));

    Ok(route_post.boxed().into())
}

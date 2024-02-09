use chrono::{Datelike, Utc};
use mongodb::bson;
use once_cell::sync::Lazy;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
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
use std::hash;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use geohash::{encode, Coord};

use mongodb::{bson::{doc, Document}, error::Error};

use reqwest;

struct Handler;

#[derive(Deserialize, Serialize)]
struct MessageAtWelcome {
    author: String,
    author_id: String,
    content: String,
    content_id: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct req {
    d: String,
    l: String,
    c: String,
    lat: Option<f64>,
    lng: Option<f64>,
    hash: Option<String>,
}


#[derive(Deserialize, Serialize)]
struct res {
    t: i8,
    r: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct User {
    discord_id: String,
    attendance: Vec<String>, // Assuming the attendance field stores dates as strings
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

// async fn get_discord_me(token: String) -> Result<String, Box<dyn std::error::Error>> {
//     let url = "https://discord.com/api/v10/users/@me";

//     let client = reqwest::Client::new();
//     let response = client.get(url)
//         .header("Authorization", format!("Bearer {}", token))
//         .send()
//         .await?;

//     if response.status().is_success() {
//         let user_data: serde_json::Value = response.json().await?;
//         if let Some(user_id) = user_data.get("id") {
//             return Ok(user_id as String)
//         } else {
//             println!("User ID not found in response.");
//         }
//     } else {
//         println!("Failed to fetch user ID: {}", response.status());
//     }

//     Ok(())
// }


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

fn gen_ran_str() -> String {
    let today = Utc::now();
    let seed = today.year() * 10000 + (today.month() as i32) * 100 + (today.day() as i32);
    let mut rng = StdRng::seed_from_u64(seed as u64);

    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let random_string: String = (0..10)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    random_string
}

fn c(result: &str) -> Result<String, std::num::ParseIntError> {
    let bytes_result: Result<Vec<u8>, _> = (0..result.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&result[i..i + 2], 16))
        .collect();

    bytes_result.map(|bytes| bytes.iter().map(|&b| (b ^ 5) as char).collect())
}

#[shuttle_runtime::main]
async fn warp(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    // task::spawn(serenity(secret_store));

    let static_files = warp::fs::dir("src/dist");
    let assets_files = warp::fs::dir("src/dist/assets");

    let route_post = warp::post()
        .and(warp::path("message"))
        .and(warp::path::param::<String>())
        .and(warp::body::json())
        .and_then(|content_id, mut message: MessageAtWelcome| async move {
            message.content_id = Some(content_id);
            SHARED_ARRAY.add_element(message);
            Ok::<_, warp::Rejection>(warp::reply::html("Sent"))
        })
        .or(warp::get().and(warp::path::end().and(static_files)))
        .or(warp::path("assets").and(assets_files))
        .or(warp::path("get_time").map(|| {
            return warp::reply::html(gen_ran_str());
        }))
        .or(warp::post()
            .and(warp::path("attendance"))
            .and(warp::body::json())
            .and_then(|mut request: req| async move {

                if let Ok(x) = c(&request.c) {
                    request.c = x;
                }
                if let Ok(x) = c(&request.d) {
                    request.d = x;
                }
                let tokens: Vec<&str> = request.l.split('|').collect();

                if let Ok(lat_str) = c(tokens[0]) { 
                    if let Some(lat) = lat_str.parse::<f64>().ok() {
                        request.lat = Some(lat);
                    } else {
                        println!("Error parsing latitude");
                    }
                }

                if let Ok(lng_str) = c(tokens[1]) { 
                    if let Some(lng) = lng_str.parse::<f64>().ok() {
                        request.lng = Some(lng);
                    } else {
                        println!("Error parsing longitude");
                    }
                }

                if let (Some(lat), Some(lng)) = (request.lat, request.lng) {
                    let n_cord = Coord { x: lng, y: lat };
                    if let Ok(hash)= encode(n_cord, 7usize){
                        request.hash = Some(hash.clone());
                        if(hash == "tuutw1s" || hash == "tuutw1t" || hash == "tuutw1v" || hash == "tuutw1u" || hash == "tuutw1k" || hash == "tuutw1m"){
                                if let Ok(client) = mongodb::Client::with_uri_str("mongodb+srv://webclub:B0m4zFTGkgvxTMfj@attendance.t5qakth.mongodb.net/?retryWrites=true&w=majority").await{
                                let db = client.database("attendance");
                                let collection = db.collection("users");
                                let new_user = User {
                                    discord_id: "new_user_id".to_string(),
                                    attendance: vec![], 
                                };
                            
                                if let Ok(user_doc) = bson::to_document(&new_user){
                                    // Insert the new user document into the collection
                                    match collection.insert_one(user_doc, None).await {
                                        Ok(_) => return Ok::<_, warp::Rejection>(warp::reply::json(&res{
                                            t: 1,
                                            r: "Success".to_string()
                                        })),
                                        Err(e) => eprintln!("Error adding new user: {}", e),
                                    }
                                }
                            }
                        }
                        // request.hash = Some(hash);
                    }
                }
                return Ok::<_, warp::Rejection>(warp::reply::json(&res{
                    t: 0,
                    r: "Error".to_string()
                }))
            }));

    Ok(route_post.boxed().into())
}

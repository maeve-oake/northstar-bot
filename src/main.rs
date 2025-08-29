use std::env;

use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::{Configuration, StandardFramework};
use serenity::all::Command;
use serenity::all::{Interaction, CreateInteractionResponseMessage, CreateInteractionResponse};
use serenity::model::channel::Message;
use serenity::all::{ActivityData, Ready};
use serenity::all::Guild;
use serenity::all::OnlineStatus;
use serenity::prelude::*;

mod commands;

use crate::commands::links::*;
use crate::commands::lists::*;
use crate::commands::northstar::*;
use crate::commands::prefixes::*;
use crate::commands::titancoins::*;

static DEFAULTPREFIX: &str = ",";
static MS: &str = "https://northstar.tf";

#[group("GENERAL")]
#[commands(prefix)]
struct General;

#[group("LIST")]
#[commands(maps, modes, playlistvars, help)]
struct List;

#[group("LINKS")]
#[commands(birb, github, wiki, info)]
struct Link;

#[group("NORTHSTAR")]
#[commands(status, search)]
struct Northstar;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
        let guilds = ctx.cache.guilds().len();
        println!("The bot is in {} guilds", guilds);
        let _guild_command = Command::create_global_command(&ctx.http, commands::titancoins::register())
        .await;

        set_activity(ctx).await;
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild, _is_new: Option<bool>) {
        new_server_reg(guild.id.get()).await.expect("fuck");
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "/redeem" {
            if let Err(why) = redeem(&ctx, &msg).await {
                println!("Error sending message: {:?}", why);
            }
        }
        if msg.content.contains("<@925064195186233344>") {
            if let Err(why) = msg.reply_ping(ctx, "what").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {

            let content = match command.data.name.as_str() {
                "redeem" => commands::titancoins::run(&command.data.options),
                _ => ":(".to_string(),
            };

            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(content)
            );

            if let Err(why) = command.create_response(&ctx.http, response).await {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

async fn set_activity(ctx: Context) {
    let activity = ActivityData::playing("Northstar.TF");
    let status = OnlineStatus::Online;
    ctx.set_presence(Some(activity), status);
}

#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "-dev" {
            println!("--- Dev mode ---");
            println!("Panic logging occurs now");
        } else {
            std::panic::set_hook(Box::new(|_info| {}));
        }
    }

    let config = Configuration::new()
        .dynamic_prefix(|_, msg| Box::pin(async move { check_db_prefix(msg.guild_id) }))
        .prefix("");

    let framework = StandardFramework::new();
    framework.configure(config);

    let framework = framework
        .group(&GENERAL_GROUP)
        .group(&LIST_GROUP)
        .group(&LINK_GROUP)
        .group(&NORTHSTAR_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

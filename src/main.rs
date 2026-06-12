use std::env;

use serenity::all::Command;
use serenity::all::OnlineStatus;
use serenity::all::{ActivityData, Ready};
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage, Interaction};
use serenity::async_trait;
use serenity::prelude::*;

mod commands;

use crate::commands::links::*;
use crate::commands::lists::*;
use crate::commands::northstar::*;
use crate::commands::titancoins::*;

static MS: &str = "https://northstar.tf";

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
        let guilds = ctx.cache.guilds().len();
        println!("The bot is in {} guilds", guilds);
        let _guild_command =
            Command::create_global_command(&ctx.http, commands::titancoins::register()).await;
        let _guild_command =
            Command::create_global_command(&ctx.http, commands::links::register()).await;
        let _guild_command =
            Command::create_global_command(&ctx.http, commands::lists::register()).await;
        let _guild_command =
            Command::create_global_command(&ctx.http, commands::northstar::register()).await;

        set_activity(ctx).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let message = match command.data.name.as_str() {
                "redeem" => CreateInteractionResponseMessage::new()
                    .content(commands::titancoins::run(&command.data.options)),

                "info" => CreateInteractionResponseMessage::new()
                    .embed(commands::links::info(&command.data.options)),

                "github" => CreateInteractionResponseMessage::new()
                    .content(commands::links::github(&command.data.options)),

                "wiki" => CreateInteractionResponseMessage::new()
                    .content(commands::links::wiki(&command.data.options)),

                "host" => CreateInteractionResponseMessage::new()
                    .content(commands::links::host(&command.data.options)),

                "help" => CreateInteractionResponseMessage::new()
                    .content(commands::lists::help(&command.data.options)),

                "maps" => CreateInteractionResponseMessage::new()
                    .content(commands::lists::maps(&command.data.options)),

                "modes" => CreateInteractionResponseMessage::new()
                    .content(commands::lists::modes(&command.data.options)),

                "playlistvars" => CreateInteractionResponseMessage::new()
                    .content(commands::lists::playlistvars(&command.data.options)),

                "status" => CreateInteractionResponseMessage::new()
                    .content(commands::northstar::status(&command.data.options).await),
                    
                "search" => CreateInteractionResponseMessage::new()
                    .content(commands::northstar::search(&command.data.options).await),

                _ => CreateInteractionResponseMessage::new().content(":("),
            };

            let response = CreateInteractionResponse::Message(message);

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

    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

use serenity::all::*;
use serenity::framework::standard::CommandResult;

use serenity::all::CommandDataOption;
use serenity::all::CreateCommand;

pub fn info(_options: &[CommandDataOption]) -> CreateEmbed {
    CreateEmbed::new()
        .title("Northstar Servers Bot")
        .description(
            "Made by maeve-oake
Remade in Rust by H0L0

A discord bot that displays the status of the northstar.tf servers

",
        )
        .field("maeve-oake", "https://miaow.ing", false)
        .field("H0L0", "https://h0l0.cc", false)
        .color(Color::from_rgb(244, 32, 105))
        .thumbnail("https://northstar.tf/assets/logo_1k.png")
        .url("https://github.com/maeve-oake/northstar-bot")
}

pub fn github(_options: &[CommandDataOption]) -> String {
    "https://github.com/R2Northstar".to_string()
}

pub fn wiki(_options: &[CommandDataOption]) -> String {
    "https://r2northstar.gitbook.io/r2northstar-wiki/".to_string()
}

pub fn host(_options: &[CommandDataOption]) -> String {
    "https://youtu.be/EZ3w2Nl9SZo".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("info").description("display info about the bot");
    CreateCommand::new("github").description("links the northstar github");
    CreateCommand::new("wiki").description("links the northstar wiki");
    CreateCommand::new("host").description("links hummusbird's server tutorial")
}

use serenity::all::*;
use serenity::framework::standard::CommandResult;

use serenity::all::CommandDataOption;
use serenity::all::CreateCommand;

// #[command]
// #[aliases(video, vid, host)]
// async fn birb(ctx: &Context, msg: &Message) -> CommandResult {
//     msg.channel_id
//         .say(ctx, "https://youtu.be/EZ3w2Nl9SZo")
//         .await?;
//     Ok(())
// }

// #[command]
// async fn wiki(ctx: &Context, msg: &Message) -> CommandResult {
//     msg.channel_id
//         .say(ctx, "https://r2northstar.gitbook.io/r2northstar-wiki/")
//         .await?;
//     Ok(())
// }

// #[command]
// #[aliases(git)]
// async fn github(ctx: &Context, msg: &Message) -> CommandResult {
//     msg.channel_id
//         .say(ctx, "https://github.com/R2Northstar")
//         .await?;
//     Ok(())
// }

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

pub fn register() -> CreateCommand {
    CreateCommand::new("info").description("display info about the bot")
}

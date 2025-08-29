use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::all::*;

#[command]
#[aliases(video, vid, host)]
async fn birb(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(ctx, "https://youtu.be/EZ3w2Nl9SZo")
        .await?;
    Ok(())
}

#[command]
async fn wiki(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(ctx, "https://r2northstar.gitbook.io/r2northstar-wiki/")
        .await?;
    Ok(())
}

#[command]
#[aliases(git)]
async fn github(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(ctx, "https://github.com/R2Northstar")
        .await?;
    Ok(())
}

#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let embed = CreateEmbed::new()
        .title("Northstar Servers Bot")
        .description(
"Made by hummusbird
Remade in Rust by H0L0

A discord bot that displays that status of the northstar.tf servers

",
        )
        .field("hummusbird", "https://birb.cc", false)
        .field("H0L0", "https://h0l0.cc", false)
        .color(Color::from_rgb(244, 32, 105))
        .thumbnail("https://northstar.tf/assets/logo_1k.png")
        .url("https://github.com/hummusbird/northstar-bot");

    let message = CreateMessage::new().content("").embed(embed);

    msg.channel_id.send_message(ctx, message).await?;
    Ok(())
}

use std::time::Duration;

use rand::Rng;
use serenity::framework::standard::CommandResult;
use serenity::all::*;
use tokio::time::sleep;

use serenity::all::CreateCommand;
use serenity::all::CommandDataOption;

pub async fn redeem(ctx: &Context, msg: &Message) -> CommandResult {
    let redeem_amount: i32 = rand::rng().random_range(50..250);
    let redeemedmsg = msg
        .reply_ping(
            ctx,
            "```diff
+"
            .to_owned()
                + &redeem_amount.to_string()
                + " TitanTokens```
    ",
        )
        .await?;
    sleep(Duration::from_millis(5000)).await;
    msg.delete(ctx).await.expect("failed to delete message");
    redeemedmsg
        .delete(ctx)
        .await
        .expect("failed to delete message");
    Ok(())
}


pub fn run(_options: &[CommandDataOption]) -> String {
    let redeem_amount: i32 = rand::rng().random_range(50..250);
"```diff
+"
        .to_owned()
            + &redeem_amount.to_string()
            + " ScorchBucks```
"
}

pub fn register() -> CreateCommand {
    CreateCommand::new("redeem").description("redeem your scorchbucks")
}

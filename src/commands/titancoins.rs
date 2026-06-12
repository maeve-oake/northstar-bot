use std::time::Duration;

use rand::Rng;
use serenity::all::*;
use serenity::framework::standard::CommandResult;
use tokio::time::sleep;

use serenity::all::CommandDataOption;
use serenity::all::CreateCommand;

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

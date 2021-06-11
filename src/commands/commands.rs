use teloxide::utils::command::BotCommand;
#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Ban a user")]
    Ban,
    #[command(description = "Unbans a user")]
    Unban,
    #[command(description = "Mute a user")]
    Mute,
    #[command(description = "Unmute a user")]
    Unmute,
}
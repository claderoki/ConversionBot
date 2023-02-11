use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use std::error::Error;

#[async_trait]
pub trait ApplicationCommand
where
    Self: Sync + Send,
{
    fn get_name(&self) -> String;
    async fn run(&self, context: CommandContext) -> CommandResult;
    fn register(&self, command: &mut CreateApplicationCommand);
}

pub struct CommandContext {
    pub command: ApplicationCommandInteraction,
}

pub type CommandError = Box<dyn Error + Send + Sync>;
pub type CommandResult<T = ()> = std::result::Result<T, CommandError>;

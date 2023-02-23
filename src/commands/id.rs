use crate::slash::{ApplicationCommand, CommandContext, CommandResult};
use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;

#[derive(Debug)]
pub struct IdCommand;

#[async_trait]
impl ApplicationCommand for IdCommand {
    fn get_name(&self) -> String {
        "id".into()
    }

    async fn run(&self, context: CommandContext) -> CommandResult {
        let option = context
            .command
            .data
            .options
            .get(0)
            .expect("Expected user option")
            .resolved
            .as_ref()
            .expect("Expected user object");

        if let CommandDataOptionValue::User(user, _member) = option {
            format!("{}'s id is {}", user.tag(), user.id)
        } else {
            "Please provide a valid user".to_string()
        };
        Ok(())
    }

    fn register(&self, command: &mut CreateApplicationCommand) {
        command
            .name("id")
            .description("Get a user id")
            .create_option(|option| {
                option
                    .name("id")
                    .description("The user to lookup")
                    .kind(CommandOptionType::User)
                    .required(true)
            });
    }
}

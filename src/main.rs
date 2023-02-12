mod commands;
mod core;
mod slash;

use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use crate::commands::id::IdCommand;
use crate::core::conversion::{ConversionRequest, ConversionService, Measurement};
use crate::slash::{ApplicationCommand, CommandContext};
use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler {
    commands: HashMap<String, Box<dyn ApplicationCommand>>,
    conversion_service: ConversionService,
}

impl Handler {
    pub fn new() -> Self {
        let commands: Vec<Box<dyn ApplicationCommand>> = vec![Box::new(IdCommand)];

        Self {
            commands: commands.into_iter().map(|c| (c.get_name(), c)).collect(),
            conversion_service: ConversionService::new(vec![]),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, message: Message) {
        if let Ok(contexts) = self
            .conversion_service
            .search(message.content.to_lowercase().as_str())
        {
            let mut context_currencies: Option<Vec<Arc<Measurement>>> = None;
            for context in contexts {
                if context.measurement.kind.is_currency() {
                    if context_currencies.is_none() {
                        // cached context currencies
                        context_currencies = Some(Vec::new());
                    } else {
                        // add currencies to list.
                    }
                }

                let request = ConversionRequest {
                    from: context.measurement,
                    value: context.value,
                    to_list: vec![],
                };

                if let Ok(_conversion) = self.conversion_service.convert(request) {}
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(env::var("GUILD_ID").unwrap().parse().unwrap());
        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            for cmd in self.commands.values() {
                commands.create_application_command(|command| {
                    cmd.register(command);
                    command
                });
            }
            commands
        })
        .await;

        println!("I now have the following guild slash commands: {commands:#?}");
    }

    async fn interaction_create(&self, _ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {command:#?}");
            let name = command.data.name.clone();
            let context = CommandContext { command };

            if let Some(cmd) = self.commands.get(&name) {
                if let Err(why) = cmd.run(context).await {
                    println!("Slash command error: {why:?}");
                }
            }
        }
    }
}

mod envhelper {
    use std::env;

    pub fn load() {
        std::fs::read_to_string(".env")
            .expect("No env file found.")
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .filter_map(|x| x.split_once('='))
            .for_each(|l| env::set_var(l.0, l.1));
    }
    pub fn validate() {
        env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        env::var("GUILD_ID")
            .expect("Expected GUILD_ID in environment")
            .parse::<u64>()
            .expect("GUILD_ID must be an integer");
    }
}

#[tokio::main]
async fn main() {
    envhelper::load();
    envhelper::validate();

    let token = env::var("DISCORD_TOKEN").unwrap();

    let mut client = Client::builder(token, GatewayIntents::MESSAGE_CONTENT)
        .event_handler(Handler::new())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

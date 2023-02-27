mod commands;
mod core;
mod slash;
mod tests;
mod data;

use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use crate::commands::id::IdCommand;
use crate::core::conversion::{ConversionContext, ConversionRequest, ConversionService, Measurement};
use crate::slash::{ApplicationCommand, CommandContext};
use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use sqlx::{MySqlPool};
use crate::data::{load_currencies, load_units};

#[derive(Debug)]
struct Handler {
    commands: HashMap<String, Box<dyn ApplicationCommand>>,
    conversion_service: ConversionService,
}

impl Handler {
    pub fn new(measurements: Vec<Measurement>) -> Self {
        let commands: Vec<Box<dyn ApplicationCommand>> = vec![Box::new(IdCommand)];

        Self {
            commands: commands.into_iter().map(|c| (c.get_name(), c)).collect(),
            conversion_service: ConversionService::new(measurements),
        }
    }
}

struct CachedCurrencies {
    currencies: Vec<Arc<Measurement>>,
    filled: bool
}

impl CachedCurrencies {
    pub fn is_filled(&self) -> bool {
        self.filled
    }

    pub fn add(&mut self, measurements: Vec<Arc<Measurement>>) {
        measurements.iter().for_each(|m|self.currencies.push(m.clone()));
        self.filled = true;
    }

    pub fn add_to_list(&self, list: &mut Vec<Arc<Measurement>>) {
        self.currencies.iter().for_each(|c| list.push(c.clone()));
    }
}

async fn get_context_currencies() -> Vec<Arc<Measurement>> {
    vec![]
}

async fn process_context(cached_currencies: &mut CachedCurrencies, context: ConversionContext) -> ConversionRequest {
    let mut to_list = vec![];
    if context.measurement.kind.is_currency() {
        if !&cached_currencies.is_filled() {
            cached_currencies.add(get_context_currencies().await);
        } else {
            cached_currencies.add_to_list(&mut to_list);
        }
    }

    ConversionRequest {
        from: context.measurement,
        value: context.value,
        to_list,
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, message: Message) {
        println!("Message received.");
        if let Ok(contexts) = self
            .conversion_service
            .search(message.content.to_lowercase().as_str())
        {
            let mut cached_currencies = CachedCurrencies { currencies: vec![], filled: false };
            for context in contexts {
                let request = process_context(&mut cached_currencies, context).await;
                if let Ok(_conversion) = self.conversion_service.convert(request) {

                }
            }
        } else {
            println!("No conversions found.");
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(env::var("GUILD_ID").unwrap().parse().unwrap());
        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            for cmd in self.commands.values() {
                commands.create_application_command(|command| {
                    cmd.register(command);
                    command
                });
            }
            commands
        })
        .await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {command:#?}");
            let name = command.data.name.clone();
            let context = CommandContext { command, ctx };

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
        env::var("DATABASE_URL").expect("Expected a DATABASE_URL in the environment");
        env::var("GUILD_ID")
            .expect("Expected GUILD_ID in environment")
            .parse::<u64>()
            .expect("GUILD_ID must be an integer");
    }
}

async fn get_measurements() -> Result<Vec<Measurement>, String> {
    let pool = MySqlPool::connect(env::var("DATABASE_URL").unwrap().as_str())
        .await
        .map_err(|_|"Error")?;

    Ok(load_units(&pool)
        .await?
        .into_iter()
        .chain(load_currencies(&pool).await?)
        .collect())
}

#[tokio::main]
async fn main() {
    envhelper::load();
    envhelper::validate();

    let token = env::var("DISCORD_TOKEN").unwrap();

    let handler = Handler::new(get_measurements().await.unwrap());
    let mut client = Client::builder(token, GatewayIntents::GUILD_MESSAGES)
        .event_handler(handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

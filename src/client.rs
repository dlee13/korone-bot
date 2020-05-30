use std::env::var;
use futures_util::stream::StreamExt;
use twilight::{
    command_parser::{Command, CommandParserConfig, Parser},
    gateway::{Event, Shard, ShardConfig},
    http::Client as HttpClient,
    model::{
        channel::message::Message,
        gateway::GatewayIntents,
        id::*,
    },
};
use crate::Result;

pub async fn start() -> Result<()> {
    let discord_token = var("DISCORD_TOKEN")?;

    let http = HttpClient::new(&discord_token);

    let info = get_current_application_info(&http).await?;

    let parser = {
        let mut config = CommandParserConfig::new();
        config.command("help").add();
        config.command("about").add();
        config.command("quit").add();

        config.add_prefix(if cfg!(debug_assertions) { "|" } else { "\\" });
        config.add_prefix(format!("<@{}>", info.id));
        config.add_prefix(format!("<@!{}>", info.id));
        Parser::new(config)
    };

    let config = {
        let mut config = ShardConfig::builder(&discord_token);
        config.intents(Some(GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES));
        config.build()
    };

    let shard = Shard::new(config).await?;

    let mut events = shard.events().await;

    let ctx = Context::new(&http, &info.owner.id, &parser, &shard);

    while let Some(event) = events.next().await {
        match handle_event(event, &ctx).await {
            Ok(_) => (),
            Err(why) if why.to_string() == "quit" => break,
            Err(why) => {
                println!("Error while handling event: {}", why);
            }
        }
    }

    Ok(())
}

async fn handle_event(event: Event, ctx: &Context<'_>) -> Result<()> {
    match event {
        Event::MessageCreate(msg) => {
            if msg.0.author.bot {
                // Don't react if message was sent by a bot.
                return Ok(());
            }

            handle_command(msg.0, &ctx).await?;
            Ok(())
        }
        _ => Ok(()),
    }
}

async fn handle_command(msg: Message, ctx: &Context<'_>) -> Result<()> {
    if let Some(command) = ctx.parser.parse(&msg.content) {
        match command {
            Command { name: "about", .. } => {
                ctx.http
                    .create_message(msg.channel_id)
                    .content("I'm a bot")?
                    .await?;
                Ok(())
            }
            Command { name: "quit", .. } => {
                if msg.author.id == *ctx.owner {
                    Err("quit".into())
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    } else {
        // No command
        Ok(())
    }
}

struct Context<'a> {
    pub http: &'a HttpClient,
    pub owner: &'a UserId,
    pub parser: &'a Parser<'a>,
    pub shard: &'a Shard,
}

impl<'a> Context<'a> {
    fn new(
        http: &'a HttpClient,
        owner: &'a UserId,
        parser: &'a Parser<'_>,
        shard: &'a Shard,
    ) -> Self {
        Context {
            http,
            owner,
            parser,
            shard,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct User {
    pub id: UserId,
    #[serde(rename = "username")]
    pub name: String,
    pub discriminator: String,
    #[serde(skip)]
    pub _nonexhaustive: (),
}

#[derive(Debug, serde::Deserialize)]
struct CurrentApplicationInfo {
    pub id: UserId,
    pub name: String,
    pub owner: User,
    #[serde(skip)]
    pub _nonexhaustive: (),
}

async fn get_current_application_info(http: &HttpClient) -> Result<CurrentApplicationInfo> {
    const PATH: &str = "oauth2/applications/@me";
    let request = twilight::http::request::Request {
        body: None,
        form: None,
        headers: None,
        method: reqwest::Method::GET,
        path: twilight::http::routing::Path::GatewayBot,
        path_str: std::borrow::Cow::Borrowed(PATH),
    };
    let response = http.raw(request).await?;
    let info = response.json::<CurrentApplicationInfo>().await?;

    Ok(info)
}

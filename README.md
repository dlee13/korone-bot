# Korone Bot

Korone is a general purpose bot with reminders as the main utility.

There is no functionality related to Vtubers (for now, at least). It's just named after someone I like.

## Getting Started

Clone this repository and create file `.env` in the root directory. Fill it with the following stub:

```env
DATABASE_PATH=korone.db
DISCORD_TOKEN=
RUST_LOG=debug
```

Then go to the [Discord Developer Portal](https://discordapp.com/developers/applications) and create an application. Add a bot and copy its access token into your `.env` file.

In the Discord Developer Portal again, use the `OAuth2 URL Generator` to make an invitation link for your bot. Select only `bot` in the scopes and invite the bot to a Discord server that you manage.

You should then be ready to compile and try running it.

### Prerequisites

```
Discord account
Rust 1.40+ with Cargo
```

## Built With

<!--
* [sled](https://github.com/spacejam/sled) - A modern embedded database
* [twilight](https://github.com/twilight-rs/twilight) - The Rust library for the Discord API
-->

## License

This project is licensed under the ISC License - see the [LICENSE.md](LICENSE.md) file for details

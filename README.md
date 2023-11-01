# Cooler Alerts: Discord Bot for Blockchain State Monitoring

Welcome to Cooler Alerts, a rust-based discord bot that monitores on-chain state and facilitates user subscriptions for custom alerts. Developed with `ethers-rs`, `serenity`, and `sqlx`, this framework is designed for easy extensibility and customization, allowing developers to fulfill diverse monitoring needs.

## Features

Cooler Alerts offers a range of functionalities, split across different modules for optimal organization and efficiency:

- **Discord Module**: Utilizing `serenity`, this module is the heart of Discord interaction. It sets up and manages slash commands, ensuring seamless communication with the Discord API.
- **Registry Module**: Leveraging `sqlx`, this module takes care of user alert registrations and management. It provides comprehensive methods for interacting with the bot's database, ensuring efficient data handling.
- **Listener Module**: Built on `ethers-rs`, this module actively monitors on-chain state. It triggers alerts for subscribed users when specific conditions are met. This can easily be expanded, allowing integration of new methods such as listening to additional on-chain events, monitoring mempools, or even integrating off-chain data feeds. For alerting users, the Listener Module supports two distinct methods: webhook notifications and email alerts.

## Using the bot

The Cooler Alerts bot has been designed so that users of the Olympus Protocol can easily setup reminders to manage their Cooler Loans before expiration. Alerts can be configured to be sent either by discord webhook or by email.

Since Cooler Loans are not time sensible because of their fix-term nature, the state monitoring cadence is set to 12h. 

The bot has 3 different slash commands:
- `create_alert`: Used to store new alerts into the DB. Has the following parameters:
   - `cooler`: Address of the Cooler contract to be monitored.
   - `loan_id`: ID of the loan to be monitored.
   - `threshold`: Days before expiration that the user should be notified in advance.
   - `webhook_url` (optional): URL of the discord webhook where the alerts should be sent. _Example: https://discord.com/api/webhooks/123/XXX_.
   - `email` (optional): Email address where the alerts should be sent. _Example: cooler_alerts@yxz.com_.
- `list_alerts`: Used to list all the existing alerts user in the DB. Only lists those registered by the user executed the slash command.
- `delete_alerts`: Used to delete user alerts of a given Cooler contract. Has the following parameters:
   - `cooler`: Address of the Cooler contract to be deleted.
   - `loan_id` (optional): ID of the loan to be deleted. If not informed, all the alerts for that Cooler contracts will be deleted.

## Developer Quick Start Guide

To get Cooler Alerts up and running, follow these simple steps:

1. Create your own discord bot.
   - Check [this tutorial](https://discordjs.guide/preparations/setting-up-a-bot-application.html#creating-your-bot) to create your own discord bot and get its token.
   - Check [this tutorial](https://discordjs.guide/preparations/adding-your-bot-to-servers.html#bot-invite-links) to add the bot to your personal server.
2. Create a new gmail account (using an existing one is not recommended as if you make the `.env` file public by mistake anyone could use that email) and enable the app password.
   - Check [this tutorial](https://support.google.com/accounts/answer/185839?hl=en) to enable 2FA.
   - Check [this tutorial](https://support.google.com/mail/answer/185833?hl=en) to create an app password.
3. Download the [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) and setup a new database with the following commands:
   ```
   sqlx database create --database-url sqlite:NAME_OF_YOUR_DB.sqlite
   sqlx database setup --database-url sqlite:NAME_OF_YOUR_DB.sqlite
   sqlx migrate run
   ```
4. **Configure the Bot**: Start by setting the `.env` file based off `example.env`, which is located at the root of the repository. This file contains essential settings that control the bot's functionalities, including the Discord API token, database connection string, the RPC connection, or the email credentials.

5. **Install Dependencies**: Run `cargo build` and ensure that the necessary libraries listed on `Cargo.toml` have been successfully installed.

6. **Run the Bot**: Once the app is built and configured, use `cargo run` to activate the bot and start sending alerts to your Discord server.

## Contributing

Contributions are welcome! If you have ideas for new features, improvements, or bug fixes, feel free to submit a pull request or open an issue.

## License

Cooler Alerts is released under Apache 2.0 license. Feel free to use, modify, and distribute as per the license terms.

Happy Monitoring!

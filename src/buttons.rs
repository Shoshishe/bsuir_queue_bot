use std::error::Error;

use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    prelude::Requester,
    sugar::bot::BotMessagesExt,
    types::{
        CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, InlineQuery,
        InlineQueryResultArticle, InputMessageContent, InputMessageContentText, Me, Message,
    },
    utils::command::BotCommands,
};

#[derive(BotCommands)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Display info about the bot and it's interactions. For now it simply writes this comments with command names
    Help,
    /// I mean, I am too lazy to write more specific for now
    Submit,
}

const KEYBOARD_SIZE: usize = 4;

fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let options = ["OOP", "SCI", "MNA", "ACS"];

    for _ in options.chunks(KEYBOARD_SIZE) {
        let row = options
            .iter()
            .map(|&option| InlineKeyboardButton::callback(option.to_owned(), option.to_owned()))
            .collect();
        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
            }
            Ok(Command::Submit) => {
                let keyboard = make_keyboard();
                bot.send_message(msg.chat.id, "Disciplines")
                    .reply_markup(keyboard)
                    .await?;
            }
            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found").await?;
            }
        }
    }
    Ok(())
}
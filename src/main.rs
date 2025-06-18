pub mod buttons;
use dotenv::dotenv;
use teloxide::prelude::*;

use crate::buttons::{callback_handler, inline_queries, message_handler};

#[tokio::main]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");
    let bot = Bot::from_env();

    let handler = dptree::entry().branch(
        Update::filter_message().endpoint(message_handler).branch(
            Update::filter_callback_query()
                .endpoint(callback_handler)
                .branch(Update::filter_inline_query().endpoint(inline_queries)),
        ),
    );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

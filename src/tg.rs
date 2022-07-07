use crate::db;
use crate::hn;
use chrono::Utc;
use log::*;
use rusqlite::*;
use teloxide::prelude::*;
use teloxide::types::*;

const RESENT_COOLDOWN: i64 = 60 * 60 * 24 * 3; // 3 days

pub struct Sender {
    bot: Bot,
    channel: String,
}

impl Sender {
    pub fn new(token: String, channel: String) -> Sender {
        Sender {
            bot: Bot::new(token),
            channel,
        }
    }
}

pub async fn send_items(sender: &Sender, conn: &Connection, items: &[hn::Item]) {
    let now = Utc::now().timestamp();
    for item in items.iter() {
        let last_sent_epoch = db::query_last_sent(&conn, &item);
        let should_send = match last_sent_epoch {
            Some(p) => p + RESENT_COOLDOWN < now,
            None => true,
        };
        if should_send {
            let message = format_item_to_message(&item);
            match send_message(sender, &message).await {
                Ok(_) => {}
                Err(_) => {
                    info!("Possibly encountered rate limit, skipping");
                    break;
                }
            };
            db::update_last_sent(&conn, &item, now)
        }
    }
}

fn format_item_to_message(item: &hn::Item) -> String {
    let comments_link = hn::get_item_comments_link(&item);
    format!(
        "<a href=\"{}\"><em>{}</em></a>\n<a href=\"{}\">comments</a>",
        &item.url, &item.title, comments_link
    )
}

async fn send_message(sender: &Sender, message: &String) -> Result<Message, ()> {
    let recipient = Recipient::ChannelUsername(sender.channel.to_string());
    info!("Sending message '{message}'");
    let mut request = sender.bot.send_message(recipient, message);
    request.parse_mode = Some(ParseMode::Html);
    // request.disable_web_page_preview = Some(true);
    request.send().await.map_err(|e| {
        error!("Failed to send message: {e}");
        () // Ignores error type
    })
}

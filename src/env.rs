use log::*;

pub enum Var {
    SleepInterval,
    SqliteDbFile,
    TelegramBotToken,
    TelegramChannelName,
}

impl Var {
    fn as_str(&self) -> &'static str {
        match self {
            Var::SleepInterval => "SLEEP_INTERVAL",
            Var::SqliteDbFile => "SQLITE_DB_FILE",
            Var::TelegramBotToken => "TELEGRAM_BOT_TOKEN",
            Var::TelegramChannelName => "TELEGRAM_CHANNEL_NAME",
        }
    }
}

pub fn get(var: Var) -> String {
    let key = var.as_str();
    match std::env::var(key) {
        Ok(val) => val,
        Err(_) => {
            warn!("Warn: missing env var {key}");
            String::from("")
        }
    }
}

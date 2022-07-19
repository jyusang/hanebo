use log::*;

mod db;
mod env;
mod hn;
mod tg;
mod utils;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let mut conn = rusqlite::Connection::open(env::get(env::Var::SqliteDbFile)).unwrap();
    let sender = tg::Sender::new(
        env::get(env::Var::TelegramBotToken),
        env::get(env::Var::TelegramChannelName),
    );
    let interval = match env::get(env::Var::SleepInterval).parse() {
        Ok(val) => val,
        Err(_) => 60 * 5, // 5 minutes
    };

    if !db::table_exists(&conn) {
        info!("Create table for posts");
        db::create_table(&conn);
    }

    loop {
        info!("Time to fetch");
        let items = match hn::get_items().await {
            Ok(val) => val,
            Err(_) => {
                warn!("Failed to get items from HN");
                info!("Time to sleep");
                continue;
            },
        };
        db::insert_items(&mut conn, &items);
        tg::send_items(&sender, &conn, &items).await;
        info!("Time to sleep");
        utils::sleep(interval).await;
    }
}

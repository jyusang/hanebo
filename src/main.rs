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
        match run(&mut conn, &sender).await {
            Ok(_) => info!("Work done"),
            Err(_) => warn!("Oops, something went wrong"),
        };
        info!("Time to sleep");
        utils::sleep(interval).await;
    }
}

async fn run(conn: &mut rusqlite::Connection, sender: &tg::Sender) -> Result<(), ()> {
    let items = hn::get_items().await.map_err(|e| {
        match e {
            hn::ScrapingError::RequestError => warn!("No response from HN"),
            hn::ScrapingError::InvalidResponseError => warn!("Invalid response from HN"),
        };
        ()
    })?;
    db::insert_items(conn, &items);
    tg::send_items(&sender, &conn, &items).await;
    Ok(())
}

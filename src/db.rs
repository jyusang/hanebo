use crate::hn;
use rusqlite::*;

pub fn table_exists(conn: &Connection) -> bool {
    let count: u32 = conn
        .query_row(
            "SELECT COUNT(1) FROM sqlite_master WHERE type='table' AND name='posts';",
            [],
            |row| row.get(0),
        )
        .unwrap();
    count > 0
}

pub fn create_table(conn: &Connection) {
    conn.execute(
        "
CREATE TABLE posts (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    last_sent INT
);
",
        [],
    )
    .unwrap();
}

pub fn insert_items(conn: &mut Connection, items: &Vec<hn::Item>) {
    let tx = conn.transaction().unwrap();
    for item in items.iter() {
        tx.execute(
            "INSERT OR IGNORE INTO posts (id, title, url) VALUES (?, ?, ?);",
            [&item.id, &item.title, &item.url],
        )
        .unwrap();
    }
    tx.commit().unwrap();
}

pub fn query_last_sent(conn: &Connection, item: &hn::Item) -> Option<i64> {
    let epoch: Option<i64> = conn
        .query_row(
            "SELECT last_sent FROM posts WHERE id = ?",
            [&item.id],
            |row| row.get(0),
        )
        .unwrap();
    epoch
}

pub fn update_last_sent(conn: &Connection, item: &hn::Item, last_sent: i64) {
    conn.execute(
        "UPDATE posts SET last_sent = ? WHERE id = ?",
        params![last_sent, item.id],
    )
    .unwrap();
}

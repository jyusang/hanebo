use std::time::Duration;

pub async fn sleep(secs: u64) {
    tokio::time::sleep(Duration::from_secs(secs)).await;
}

use scraper::{Html, Selector};

pub struct Item {
    pub id: String,
    pub url: String,
}

pub async fn get_items() -> Result<Vec<Item>, ()> {
    let resp = reqwest::get("https://news.ycombinator.com/").await.unwrap();
    let html = resp.text().await.unwrap();
    let doc = Html::parse_document(&html);
    let item_selector = Selector::parse(".athing").unwrap();
    let item_link_selector = Selector::parse(".title > a").unwrap();
    let items = doc
        .select(&item_selector)
        .map(|item| {
            let link = item.select(&item_link_selector).next().unwrap();
            Item {
                id: item.value().attr("id").unwrap().to_string(),
                url: link.value().attr("href").unwrap().to_string(),
            }
        })
        .collect::<Vec<Item>>();
    Result::Ok(items)
}

pub fn get_item_hn_link(item: &Item) -> String {
    format!("https://news.ycombinator.com/item?id={}", &item.id)
}

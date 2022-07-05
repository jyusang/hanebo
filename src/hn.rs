use scraper::{Html, Selector};
use url::Url;

const HN_BASE_URL: &str = "https://news.ycombinator.com/";

pub struct Item {
    pub id: String,
    pub url: String,
}

pub async fn get_items() -> Result<Vec<Item>, ()> {
    let resp = reqwest::get(HN_BASE_URL).await.unwrap();
    let html = resp.text().await.unwrap();
    let doc = Html::parse_document(&html);
    let item_selector = Selector::parse(".athing").unwrap();
    let item_link_selector = Selector::parse(".title > a").unwrap();
    let items = doc
        .select(&item_selector)
        .map(|item| {
            let item_id = item.value().attr("id").unwrap().to_string();

            let link_element = item.select(&item_link_selector).next().unwrap();
            let item_href = link_element.value().attr("href").unwrap().to_string();
            let item_url = match Url::parse(&item_href) {
                Ok(_) => item_href,
                Err(_) => format!("{}{}", HN_BASE_URL, &item_href),
            };

            Item {
                id: item_id,
                url: item_url,
            }
        })
        .collect::<Vec<Item>>();
    Result::Ok(items)
}

pub fn get_item_hn_link(item: &Item) -> String {
    format!("{}item?id={}", HN_BASE_URL, &item.id)
}

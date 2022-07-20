use log::*;
use scraper::{ElementRef, Html, Selector};
use url::Url;

const HN_BASE_URL: &str = "https://news.ycombinator.com/";

pub enum ScrapingError {
    RequestError,
    InvalidResponseError,
}

pub struct Item {
    pub id: String,
    pub title: String,
    pub url: String,
}

pub async fn get_items() -> Result<Vec<Item>, ScrapingError> {
    let resp = reqwest::get(HN_BASE_URL)
        .await
        .map_err(|_| ScrapingError::RequestError)?;
    let html = resp
        .text()
        .await
        .map_err(|_| ScrapingError::InvalidResponseError)?;
    let doc = Html::parse_document(&html);
    let item_selector = Selector::parse(".athing").unwrap();
    let items = doc
        .select(&item_selector)
        .filter_map(|elem| parse_item(elem))
        .collect::<Vec<Item>>();
    Result::Ok(items)
}

pub fn derive_item_comments_link(item: &Item) -> String {
    format!("{}item?id={}", HN_BASE_URL, &item.id)
}

fn parse_item(elem: ElementRef) -> Option<Item> {
    let item_id = match elem.value().attr("id") {
        Some(x) => x.to_string(),
        None => {
            warn!("Failed to parse item id, skipping");
            return None;
        }
    };
    let link_selector = Selector::parse(".title > a").unwrap();
    let link_element = match elem.select(&link_selector).next() {
        Some(x) => x,
        None => {
            warn!("Failed to parse item title, skipping");
            return None;
        }
    };
    let item_title = link_element.inner_html();
    let item_href = match link_element.value().attr("href") {
        Some(x) => x.to_string(),
        None => {
            warn!("Failed to parse item link, skipping");
            return None;
        }
    };
    let item_url = match Url::parse(&item_href) {
        Ok(_) => item_href,
        Err(_) => format!("{}{}", HN_BASE_URL, &item_href),
    };
    Some(Item {
        id: item_id,
        title: item_title,
        url: item_url,
    })
}

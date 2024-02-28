use std::{cmp::Ordering, collections::BTreeMap, error::Error, str::FromStr};

use rss::{extension::Extension, Channel};
use urlencoding::encode;

use crate::{
    app::{App, Widgets},
    widget::{category::ALL_CATEGORIES, sort::Sort},
};

use super::{
    nyaa_html::{to_bytes, Item},
    Source,
};

pub struct NyaaRssSource;

type ExtensionMap = BTreeMap<String, Vec<Extension>>;

pub fn get_ext_value<T: Default + FromStr>(ext_map: &ExtensionMap, key: &str) -> T {
    ext_map
        .get(key)
        .and_then(|v| v.first())
        .and_then(|s| s.value.to_owned())
        .and_then(|val| val.parse::<T>().ok())
        .unwrap_or_default()
}

impl Source for NyaaRssSource {
    async fn sort(app: &mut App, w: &Widgets) -> Result<Vec<Item>, Box<dyn Error>> {
        let mut items = w.results.table.items.clone();
        let f: fn(&Item, &Item) -> Ordering = match w.sort.selected {
            Sort::Date => |a, b| a.index.cmp(&b.index),
            Sort::Downloads => |a, b| b.downloads.cmp(&a.downloads),
            Sort::Seeders => |a, b| b.seeders.cmp(&a.seeders),
            Sort::Leechers => |a, b| b.leechers.cmp(&a.leechers),
            Sort::Size => |a, b| b.bytes.cmp(&a.bytes),
        };
        items.sort_by(f);
        if app.reverse {
            items.reverse();
        }
        Ok(items)
    }
    async fn search(app: &mut App, w: &Widgets) -> Result<Vec<Item>, Box<dyn Error>> {
        let cat = w.category.category;
        let query = w.search.input.input.clone();
        let filter = w.filter.selected.clone() as usize;
        app.last_page = 1;
        app.page = 1;
        let (high, low) = (cat / 10, cat % 10);
        let query = encode(&query);
        let url = format!(
            "https://nyaa.si/?page=rss&f={}&c={}_{}&q={}&m",
            filter, high, low, query
        );
        let content = reqwest::get(url).await?.bytes().await?;

        let channel = Channel::read_from(&content[..])?;

        let results: Vec<Item> = channel
            .items
            .iter()
            .enumerate()
            .filter_map(|(index, item)| {
                let ext = item.extensions().get("nyaa")?;
                let guid = item.guid.to_owned().unwrap_or_default();
                let id = guid.value.split('/').last().unwrap_or_default().to_owned(); // Get nyaa id from guid url in format
                                                                                      // `https://nyaa.si/view/{id}`
                let category_str = get_ext_value::<String>(ext, "categoryId");
                let split: Vec<&str> = category_str.split('_').collect();
                let high = split.first().unwrap_or(&"1").parse::<usize>().unwrap_or(1);
                let low = split.last().unwrap_or(&"0").parse::<usize>().unwrap_or(0);
                let category = high * 10 + low;
                let icon = ALL_CATEGORIES
                    .get(high)
                    .and_then(|c| c.find(category))
                    .unwrap_or_default();
                let size = get_ext_value::<String>(ext, "size")
                    .replace('i', "")
                    .replace("Bytes", "B");

                Some(Item {
                    index,
                    seeders: get_ext_value(ext, "seeders"),
                    leechers: get_ext_value(ext, "leechers"),
                    downloads: get_ext_value(ext, "downloads"),
                    bytes: to_bytes(&size),
                    size,
                    title: item.title.to_owned().unwrap_or("???".to_owned()),
                    torrent_link: format!("https://nyaa.si/download/{}.torrent", id),
                    magnet_link: item.link.to_owned().unwrap_or("???".to_owned()),
                    file_name: format!("{}.torrent", id),
                    trusted: get_ext_value::<String>(ext, "trusted").eq("Yes"),
                    remake: get_ext_value::<String>(ext, "remake").eq("Yes"),
                    category,
                    icon,
                })
            })
            .collect();
        app.total_results = results.len();
        Ok(results)
    }

    async fn filter(app: &mut App, w: &Widgets) -> Result<Vec<Item>, Box<dyn Error>> {
        NyaaRssSource::search(app, w).await
    }

    async fn categorize(app: &mut App, w: &Widgets) -> Result<Vec<Item>, Box<dyn Error>> {
        NyaaRssSource::search(app, w).await
    }
}

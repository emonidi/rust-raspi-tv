use reqwest::blocking::Client;
use scraper::{ElementRef, Selector};
use serde::Deserialize;
use serde_json::Number;

#[derive(Debug)]
pub struct Channel {
    pub img_url: String,
    pub url: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
struct ChannelResponse {
    link: String,
    is_test: bool,
    duration: Number,
    cassandra: bool,
}

impl Channel {
    pub fn new(element: ElementRef, client: Client) -> Channel {
        let title = Channel::get_title(element);
        let img_url = Channel::get_img_url(element);
        let m3u = Channel::get_m3u(element, client);
        Channel {
            img_url,
            url: m3u,
            title,
        }
    }

    pub fn get_title(element: ElementRef) -> String {
        let selector = Selector::parse(".playlist-item__title").unwrap();
        let title = element.select(&selector).next().unwrap();
        title.text().collect::<Vec<_>>()[0].to_string()
    }

    pub fn get_img_url(element: ElementRef) -> String {
        let selector = Selector::parse(".playlist-item__icon").unwrap();
        let img = element.select(&selector).next().unwrap();
        img.value().attr("src").unwrap().to_string()
    }

    pub fn get_m3u(element: ElementRef, client: Client) -> String {
        let request_url = element
            .select(&Selector::parse("a").unwrap())
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap();

        let res = client.get(request_url).send();
        match res {
            Ok(resp) => resp.json::<ChannelResponse>().unwrap().link.to_string(),
            Err(err) => err.to_string(),
        }

        // String::from("")
    }
}

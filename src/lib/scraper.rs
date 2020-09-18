use crate::lib::channel::Channel;

use reqwest::blocking::Client;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct NScraper {
    client: Client,
}

impl NScraper {
    pub fn new() -> NScraper {
        let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();
        NScraper { client }
    }

    pub fn sign_in(&self) {
        let sign_in_get_response = self
            .client
            .get("https://neterra.tv/sign-in")
            .send()
            .unwrap();

        let sign_in_body = sign_in_get_response.text().unwrap();
        let body = scraper::Html::parse_document(&sign_in_body);
        let selector = Selector::parse("input[name='_token']").unwrap();
        let token = &body
            .select(&selector)
            .next()
            .unwrap()
            .value()
            .attr("value")
            .unwrap();

        let login_form = reqwest::blocking::multipart::Form::new()
            .text("_token", token.to_string())
            .text("username", "pchoutov")
            .text("password", "Alex2501");

        self.client
            .post("https://neterra.tv/sign-in")
            .multipart(login_form)
            .send()
            .unwrap();
    }

    pub fn get_live_channels(&self) -> Vec<Channel> {
        let mut channels: Vec<Channel> = Vec::new();
        let response = self.client.get("https://neterra.tv/live").send().unwrap();
        let response_text = response.text().unwrap();
        let document = Html::parse_document(&response_text);
        let selector = Selector::parse("li[data-favorite]").unwrap();
        for element in document.select(&selector) {
            let ch = Channel::new(element, self.client.clone());
            channels.push(ch);
        }

        return channels;
    }
}

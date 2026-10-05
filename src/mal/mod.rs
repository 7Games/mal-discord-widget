use std::process::exit;
use reqwest::header;
use scraper::{Html, Selector, Node};
use time::{Date, macros::format_description};

use crate::environment;

pub struct AnimeStats {
    pub watching: u64,
    pub completed: u64,
    pub on_hold: u64,
    pub dropped: u64,
    pub plan_to_watch: u64,

    pub total_anime: u64,
    pub days_watched: f64,

    pub joined: Date,
}

fn get_anime_days_viewed(document: &Html) -> f64 {
    let mut result: f64 = 0.0;

    let days_selector = Selector::parse("div.di-tc.al.pl8.fs12.fw-b").unwrap();

    let days: Vec<_> = document.select(&days_selector).collect();
    let anime_days = days[0];

    // For some reason anime_days looks something like `<span class="fn-grey2 fw-n">Days: </span>17.4` so we have to do some stuff to ignore the first tag
    let unwanted_part = Selector::parse("span.fn-grey2.fw-n").unwrap();

    if let Some(span_ref) = anime_days.select(&unwanted_part).next() {
        if let Some(next_sibling) = span_ref.next_sibling() {
            if let Node::Text(text_node) = next_sibling.value() {
                let days_value = text_node.trim();
                result = days_value.parse().unwrap();
            }
        }
    }

    result
}

fn get_user_joined_date(document: &Html) -> Date {
    let user_stuff_selector = Selector::parse("li.clearfix span.user-status-data.di-ib.fl-r").unwrap();

    let stuff: Vec<_> = document.select(&user_stuff_selector).collect();
    let joined = stuff[stuff.len()-1].inner_html();

    let format = format_description!("[month repr:short] [day], [year]");

    match Date::parse(&joined, &format) {
        Ok(date) => date,
        Err(_) => Date::from_calendar_date(2000, time::Month::January, 1).unwrap(),
    }
}

fn get_anime_stat(document: &Html, class: &str) -> u64 {
    let stat_selector = Selector::parse(&format!("a.di-ib.fl-l.lh10.circle.anime.{} + span", class)).unwrap();

    let stuff = document.select(&stat_selector).collect::<Vec<_>>()[0];

    match stuff.inner_html().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }
}

fn get_anime_total(document: &Html) -> u64 {
    let total_selector = Selector::parse("span.di-ib.fl-l.fn-grey2 + span").unwrap();

    let stuff = document.select(&total_selector).collect::<Vec<_>>()[0];

    match stuff.inner_html().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }
}

pub fn get_anime_stats(env: &environment::Environment) -> AnimeStats {
    let client = reqwest::blocking::Client::new();

    let res = match client.get(format!("https://myanimelist.net/profile/{}", env.mal_user_name))
        .header(header::USER_AGENT, env.mal_user_agent.clone())
        .send() {
            Ok(res) => res,
            Err(e) => {
                eprintln!("ERROR: Couldn't get anime_statistics! {e}");
                exit(1);
            },
        };

    // Error checking
    match res.status().as_u16() {
        200 => {},
        _default => {
            eprintln!("ERROR: Returned Code {}! {:?}", res.status().as_u16(), res);
            exit(1);
        },
    }

    let document = Html::parse_document(&res.text().unwrap());

    AnimeStats {
        days_watched: get_anime_days_viewed(&document),
        joined: get_user_joined_date(&document),
        watching: get_anime_stat(&document, "watching"),
        completed: get_anime_stat(&document, "completed"),
        on_hold: get_anime_stat(&document, "on_hold"),
        dropped: get_anime_stat(&document, "dropped"),
        plan_to_watch: get_anime_stat(&document, "plan_to_watch"),
        total_anime: get_anime_total(&document),
    }
}

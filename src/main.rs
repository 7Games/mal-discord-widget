use std::process::exit;
use reqwest::header;
use scraper::{Html, Selector};
use scraper::ElementRef;

mod environment;

fn main() {
    let env = environment::get_environments();

    let client = reqwest::blocking::Client::new();

    let res = match client.get(format!("https://myanimelist.net/profile/{}", env.mal_user_name))
        .header(header::USER_AGENT, env.mal_user_agent)
        .send() {
            Ok(res) => res,
            Err(e) => {
                eprintln!("ERROR: Couldn't get anime_statistics! {e}");
                exit(1);
            },
        };

    match res.status().as_u16() {
        200 => {},
        _default => {
            eprintln!("ERROR: Returned Code {}! {:?}", res.status().as_u16(), res);
            exit(1);
        },
    }

    let html = Html::parse_document(&res.text().unwrap());

    let ul = Selector::parse("ul").unwrap();
    let span = Selector::parse("span").unwrap();
    let a = Selector::parse("a").unwrap();

    let element = html
        .select(&ul)
        .find(|e| e.value().attr("class") == Some("stats-status fl-l"));

    if let Some(element) = element {
        let anime_watching = element
            .select(&a)
            .find(|e| e.value().attr("class") == Some("di-ib fl-l lh10 circle anime watching"))
            .unwrap()
            .next_sibling()
            .unwrap();

        let anime_watching = ElementRef::wrap(anime_watching);

        if let Some(anime_watching) = anime_watching {
            println!("found element: {}", anime_watching.inner_html());
        }
    } else {
        println!("ERROR: Cannot find Anime Stats");
        exit(1);
    }
}

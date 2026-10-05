use std::process::exit;
use reqwest::header;
use serde_json::Value;
use time::macros::format_description;

use crate::environment::Environment;
use crate::mal::AnimeStats;

fn discord_send_it(env: &Environment, json: Value) {
    let client = reqwest::blocking::Client::new();

    let res = match client.patch(format!("https://discord.com/api/v9/applications/{}/users/{}/identities/0/profile", env.discord_app_id, env.discord_user_id))
        .json(&json)
        .header(header::AUTHORIZATION, format!("Bot {}", env.discord_bot_token))
        .header(header::USER_AGENT, env.discord_user_agent.clone())
        .send() {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Something went wrong with the request! {e}");
                exit(1);
            }
        };


    // Error checking
    match res.status().as_u16() {
        200 => {},
        204 => {}, // apparently ok?
        _default => {
            eprintln!("ERROR: Returned Code {}! {:?}", res.status().as_u16(), res);
            exit(1);
        },
    }

    println!("OK: Sent to Discord! It's probably ok with it anyways");
}

// I was gonna do a fancy serde thingy but ehhhhhhh strings works just as well
fn construct_json(stats: &AnimeStats) -> Value {
    let format = format_description!("[month repr:short] [day], [year]");

    let json_text: String = format!(
        "{{
\"data\": {{
\"dynamic\": [
{{
\"type\":2,
\"name\":\"watching\",
\"value\":{}
}},
{{
\"type\":2,
\"name\":\"completed\",
\"value\":{}
}},
{{
\"type\":2,
\"name\":\"plantowatch\",
\"value\":{}
}},
{{
\"type\":2,
\"name\":\"dropped\",
\"value\":{}
}},
{{
\"type\":2,
\"name\":\"onhold\",
\"value\":{}
}},
{{
\"type\":2,
\"name\":\"totalanime\",
\"value\":{}
}},
{{
\"type\":1,
\"name\":\"dayswatched\",
\"value\":\"Days Watched: {}\"
}},
{{
\"type\":1,
\"name\":\"joindate\",
\"value\":\"Joined: {}\"
}}
]
}}
}}",
        stats.watching,
        stats.completed,
        stats.plan_to_watch,
        stats.dropped,
        stats.on_hold,
        stats.total_anime,
        stats.days_watched,
        stats.joined.format(&format).unwrap(),
    );

    serde_json::from_slice(&json_text.into_bytes()).unwrap()
}

pub fn send_to_discord(env: &Environment, stats: &AnimeStats) {
    discord_send_it(env, construct_json(stats));
}

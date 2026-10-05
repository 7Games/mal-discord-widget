use std::process::exit;
use dotenv::dotenv;

pub struct Environment {
    pub discord_bot_token: String,
    pub discord_app_id: String,
    pub discord_user_id: String,

    pub mal_user_name: String,

    pub discord_user_agent: String,
    pub mal_user_agent: String,
}

fn get_env(key: &str) -> String {
    match std::env::var(key) {
        Ok(str) => str,
        Err(e) => {
            eprintln!("ERROR: Couldn't get {key}, does it exist in .env? {e}");
            exit(1);
        }
    }
}

pub fn get_environments() -> Environment {
    // Get things from .env
    match dotenv() {
        Ok(_) => {},
        Err(e) => {
            eprintln!("ERROR: Couldn't get environment vars, does .env exist? {e}");
            exit(1);
        }
    }

    Environment {
        // discord
        discord_bot_token: get_env("DISC_BOT_TOKEN"),
        discord_app_id: get_env("DISC_APP_ID"),
        discord_user_id: get_env("DISC_USER_ID"),
        // my anime list
        mal_user_name: get_env("MAL_USER_NAME"),
        // misc
        discord_user_agent: get_env("MAL_USER_AGENT"),
        mal_user_agent: get_env("DISC_USER_AGENT"),
    }
}

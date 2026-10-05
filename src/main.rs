mod environment;
mod mal;
mod discord;

fn main() {
    let env = environment::get_environments();
    let stats = mal::get_anime_stats(&env);

    println!("Sending these stats to Discord:-");
    println!("Watched: {}", stats.watching);
    println!("Completed: {}", stats.completed);
    println!("Dropped: {}", stats.dropped);
    println!("On Hold: {}", stats.on_hold);
    println!("Plan to Watch: {}", stats.plan_to_watch);
    println!("Total Anime: {}", stats.total_anime);
    println!("Days Watched: {}", stats.days_watched);
    println!("Joined: {}", stats.joined);

    discord::send_to_discord(&env, &stats);
}

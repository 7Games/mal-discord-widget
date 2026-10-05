mod environment;
mod mal;

fn main() {
    let env = environment::get_environments();
    let stats = mal::get_anime_stats(&env);
}

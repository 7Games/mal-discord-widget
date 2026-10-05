mod environment;

fn main() {
    let env = environment::get_environments();

    println!("{}", env.mal_client_id);
}

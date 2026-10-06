use std::process::exit;
use std::env;

mod environment;
mod mal;
mod discord;

struct Cmdargs {
    dry_run: bool,
    help: bool,
    version: bool,
}

fn get_arguments() -> Cmdargs {
    let args: Vec<String> = env::args().collect();
    let mut result: Cmdargs = Cmdargs {
        dry_run: false,
        help: false,
        version: false,
    };

    for arg in args {
        if arg == "-h" || arg == "--help" {
            result.help = true;
        } else if arg == "-v" || arg == "--version" {
            result.version = true;
        } else if arg == "-d" || arg == "--dry" {
            result.dry_run = true;
        }
    }

    result
}

fn print_help() {
    println!("{}\n", env!("CARGO_PKG_DESCRIPTION"));
    println!("  -d, --dry\t\t\tJust take data from MAL and display");
    println!("  -h, --help\t\tShow this help then quit");
    println!("  -v, --version\t\tShow the program version then quit");
}

fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Created by {}", env!("CARGO_PKG_AUTHORS").split(":").collect::<Vec<_>>().join(", "));
    println!("Licenced under {}", env!("CARGO_PKG_LICENSE"));
    println!("Repo {}", env!("CARGO_PKG_REPOSITORY"));
}

fn main() {
    let env = environment::get_environments();
    let stats = mal::get_anime_stats(&env);

    let args = get_arguments();

    if args.version == true {
        print_version();
        exit(0);
    }
    if args.help == true {
        print_help();
        exit(0);
    }

    if !args.dry_run {
        println!("Sending these stats to Discord:-");
    } else {
        println!("DRY RUN:-");
    }

    println!("Watched: {}", stats.watching);
    println!("Completed: {}", stats.completed);
    println!("Dropped: {}", stats.dropped);
    println!("On Hold: {}", stats.on_hold);
    println!("Plan to Watch: {}", stats.plan_to_watch);
    println!("Total Anime: {}", stats.total_anime);
    println!("Days Watched: {}", stats.days_watched);
    println!("Joined: {}", stats.joined);

    if !args.dry_run {
        discord::send_to_discord(&env, &stats);
    } else {
        println!("\nDry run... didn't sent to Discord.");
    }
}

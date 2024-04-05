use clap::{value_parser, Arg, Command};
use rand::Rng;

const BLUE_COTTON_BALL: u128 = 0;
const RED_COTTON_BALL: u128 = 1;

fn main() {
    let cli = Command::new("cotton-ball-game")
        .args([Arg::new("count")
            .long("count")
            .short('c')
            .value_parser(value_parser!(usize))
            .help("how many games to run (default is 100000)")])
        .about("run the cotton ball game")
        .get_matches();

    let count: usize = cli.get_one("count").copied().unwrap_or(100000);

    let mut total_rounds: u128 = 0;

    let mut random_generator = rand::thread_rng();

    for _ in 0..count {
        let mut balls = 2;

        loop {
            let selected_ball = random_generator.gen_range(0..balls);
            if selected_ball != BLUE_COTTON_BALL {
                balls += RED_COTTON_BALL;
                continue;
            }
            break;
        }

        total_rounds += balls - 1;
    }

    println!("==== Results ============");
    println!("{} games played", count);
    println!("{} rounds played", total_rounds);
    println!("rounds per game: {}", total_rounds as f64 / count as f64);
    println!();
    println!();
}

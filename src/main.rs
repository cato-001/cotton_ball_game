use rand::Rng;

const BLUE_COTTON_BALL: u128 = 0;
const RED_COTTON_BALL: u128 = 1;
const TOTAL_GAMES: u32 = 10000000;

fn main() {
    loop {
        let mut total_rounds: u128 = 0;

        let mut random_generator = rand::thread_rng();
        for _ in 0..TOTAL_GAMES {
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
        println!("{} games played", TOTAL_GAMES);
        println!("{} rounds played", total_rounds);
        println!(
            "rounds per game: {}",
            total_rounds as f64 / TOTAL_GAMES as f64
        );
        println!();
        println!();
    }
}

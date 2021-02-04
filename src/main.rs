use getopts::Options;
use std::env;

mod lottery_actions;

fn main() {
    // Set up the rules for the games
    // See https://www.megamillions.com/how-to-play
    let megamillions = lottery_actions::LotteryRules {
        high_ball: 70,
        bonus_high_ball: 25,
        draws: 5,
        bonus_draws: 1,
    };

    // See https://www.masslottery.com/games/powerball/how-to-play
    let powerball = lottery_actions::LotteryRules {
        high_ball: 69,
        bonus_high_ball: 26,
        draws: 5,
        bonus_draws: 1,
    };

    // Set up getopts
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("p", "", "Pick Powerball Numbers");
    opts.optflag("m", "", "Pick Megamillions Numbers");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // Print usage if '-h' is called
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // Default to megamillions, and switch to powerball if '-p' is called
    let game: &lottery_actions::LotteryRules = if matches.opt_present("p") {
        &powerball
    } else {
        &megamillions
    };


    // Pick the winners
    let winning_numbers = lottery_actions::pick_em(game.high_ball, game.draws);
    let bonus_numbers = lottery_actions::pick_em(game.bonus_high_ball, game.bonus_draws);

    lottery_actions::print_winners(winning_numbers, bonus_numbers);
}

// Usage statement for getopts
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

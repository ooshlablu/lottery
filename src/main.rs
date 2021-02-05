use structopt::StructOpt;

mod lottery_actions;

//////////////////////////////////////////
// Set up the options
/////////////////////////////////////////
#[derive(StructOpt, Debug)]
#[structopt(name = "lottery")]
struct Opts {
    /// Powerball Picks
    #[structopt(short, long)]
    powerball: bool,

    /// Megamillions Picks
    #[structopt(short, long)]
    megamillions: bool,
}

fn main() {
    //////////////////////////////////////
    // Pull the options in
    //////////////////////////////////////
    let opts = Opts::from_args();


    //////////////////////////////////////////////////
    // Set up the rules for the games
    //////////////////////////////////////////////////

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


    //////////////////////////////////////i///////////
    // Switch the game based on the flags passed in
    //////////////////////////////////////////////////
    let game: &lottery_actions::LotteryRules = if opts.powerball {
        &powerball
    } else if opts.megamillions {
        &megamillions
    } else {
        &megamillions
    };


    ///////////////////////////////
    // Do the real work
    ///////////////////////////////

    // Pick the winners
    let winning_numbers = lottery_actions::pick_em(game.high_ball, game.draws);
    let bonus_numbers = lottery_actions::pick_em(game.bonus_high_ball, game.bonus_draws);

    lottery_actions::print_winners(winning_numbers, bonus_numbers);
}

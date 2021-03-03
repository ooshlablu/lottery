use structopt::StructOpt;

//////////////////////////////////////////
// Set up the options
/////////////////////////////////////////
#[derive(StructOpt, Debug)]
#[structopt(name = "lottery")]
struct Opts {
    /// Powerball Picks
    #[structopt(short, long)]
    powerball: bool,

    /// Mega Millions Picks (default)
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
    let megamillions = lottery::LotteryRules {
        high_ball: 70,
        bonus_high_ball: 25,
        draws: 5,
        bonus_draws: 1,
    };

    // See https://www.masslottery.com/games/powerball/how-to-play
    let powerball = lottery::LotteryRules {
        high_ball: 69,
        bonus_high_ball: 26,
        draws: 5,
        bonus_draws: 1,
    };


    //////////////////////////////////////i///////////
    // Switch the game based on the flags passed in
    //////////////////////////////////////////////////
    let game: lottery::LotteryRules = if opts.powerball {
        powerball
    } else if opts.megamillions {
        megamillions
    } else {
        megamillions
    };


    ///////////////////////////////
    // Do the real work
    ///////////////////////////////

    // Pick the winners
    lottery::run(game);
}

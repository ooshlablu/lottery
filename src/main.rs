// I don't think there's a game in the U.S. that doesn't follow these rules
// There should be a a pool of numbers to pick from starting at 1
// and ending at a high number, 'high_ball', with a number of 'draws'
// to pick from that pool of numbers. There should also a bonus draw,
// with a pool of numbers starting at 1 and ending 'bonus_high_ball',
// with a number of 'bonus_draws' to pick from that pool of numbers.
struct LotteryRules {
    high_ball: i32,
    bonus_high_ball: i32,
    draws: i32,
    bonus_draws: i32,
}

use rand::Rng;

fn main() {
    // See https://www.megamillions.com/how-to-play
    let megamillions = LotteryRules {
        high_ball: 70,
        bonus_high_ball: 25,
        draws: 5,
        bonus_draws: 1,
    };

    // See https://www.masslottery.com/games/powerball/how-to-play
    let powerball = LotteryRules {
        high_ball: 69,
        bonus_high_ball: 26,
        draws: 5,
        bonus_draws: 1,
    };



    // Pick the winners
    let winning_numbers = pick_em(powerball.high_ball, powerball.draws);
    let bonus_numbers = pick_em(powerball.bonus_high_ball, powerball.bonus_draws);

    print_winners(winning_numbers, bonus_numbers);
}

fn rand_pick(rand_max: i32) -> i32 {
   rand::thread_rng().gen_range(1..=rand_max)
}

fn pick_em(high_ball: i32, draws: i32) -> Vec<i32> {
    let mut picks = Vec::new();

    let mut pick_num = 0;

    // Pick the numbers
    while pick_num < draws {
        let pick = rand_pick(high_ball);

        // It is only a valid pick if it does not
        // match a number that already has been picked.
        if !picks.contains(&pick) {
            picks.push(pick);
            pick_num += 1;
        }
    };

    // Make them nice and in order
    picks.sort();
    
    picks
}

fn print_winners(winning_numbers: Vec<i32>, bonus_numbers: Vec<i32>) {
    print!("Winning numbers are ");

    // Loop through the winning and bonus numbers. Format the output
    // with a '-' between the numbers, like how it would look on
    // a printout from a lottery machine
    for winning_number in winning_numbers.iter() {
        if winning_number == winning_numbers.last().unwrap() {
            print!("{} ", winning_number);
        } else {
            print!("{}-", winning_number);
        };
    }

    for bonus_number in bonus_numbers.iter() {
        if bonus_number == bonus_numbers.last().unwrap() {
            println!("({})", bonus_number);
        } else {
            print!("({})-", bonus_number);
        };
    }
}

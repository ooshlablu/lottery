use rand::Rng;

// I don't think there's a game in the U.S. that doesn't follow these rules.
// There should be a a pool of numbers to pick from starting at 1
// and ending at a high number, 'high_ball', with a number of 'draws'
// to pick from that pool of numbers. There should also be a bonus draw,
// with a pool of numbers starting at 1 and ending 'bonus_high_ball',
// with a number of 'bonus_draws' to pick from that pool of numbers.
pub struct LotteryRules {
    pub high_ball: i32,
    pub bonus_high_ball: i32,
    pub draws: i32,
    pub bonus_draws: i32,
}

pub struct Winners {
    pub winning_numbers: Vec<i32>,
    pub bonus_numbers: Vec<i32>,
}

// Pick a random number within the range created by 'high_ball'
// Not public because the only thing using it is pick_em
fn rand_pick(rand_max: i32) -> i32 {
   rand::thread_rng().gen_range(1..=rand_max)
}

// Loop through and pick the amount of numbers we specify
pub fn pick_em(high_ball: i32, draws: i32) -> Vec<i32> {
    if  high_ball < draws  {
        panic!("You cannot create a full unique set with the high ball and number of draws you specified. {} !>= {}!", high_ball, draws);
    } else if draws < 1 {
        panic!("You cannot have less than 1 draw! Draws: {}", draws);
    }

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

// Print the winners all nicey-nicey
pub fn print_winners(winners: Winners) -> String {
    let mut winning_numbers = winners.winning_numbers;
    let mut bonus_numbers = winners.bonus_numbers;
    if winning_numbers.len() < 1 || bonus_numbers.len() < 1 {
        panic!("You passed in an invalid amount of winning or bonus numbers. Each must be a Vec<i32> with at least 1 element. Winning numbers length: {}; Bonus numbers length: {}", winning_numbers.len(), bonus_numbers.len());
    }

    let mut printout = format!("Winning numbers:\n");

    // Loop through the winning and bonus numbers. Format the output
    // with a '-' between the numbers, like how it would look on
    // a printout from a lottery machine
    let last_winner = winning_numbers.pop();
    for winning_number in &winning_numbers {
        printout += &format!("{}-", winning_number);
    }
    printout += &format!("{} ", last_winner.unwrap());

    let last_bonus = bonus_numbers.pop();
    for bonus_number in &bonus_numbers {
        printout += &format!("({})-", bonus_number);
    }
    printout += &format!("({})\n", last_bonus.unwrap());

    printout
}

pub fn run(lottery_rules: LotteryRules) {
   let winning_numbers = pick_em(lottery_rules.high_ball, lottery_rules.draws);
   let bonus_numbers = pick_em(lottery_rules.bonus_high_ball, lottery_rules.bonus_draws);

   let winners = Winners {
                     winning_numbers: winning_numbers,
                     bonus_numbers: bonus_numbers,
   };

   print!("{}", print_winners(winners));
}

/////////////////////////////////////
// Configure the tests for this mod
/////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test the random picker
    fn test_picker() {
        assert_eq!(rand_pick(1), 1);
    }

    #[test]
    #[should_panic]
    // Test an invalid range in the random picker
    fn test_picker_invalid() {
        rand_pick(0);
    }

    #[test]
    // Test to make sure pick_em returns the amount we ask it to
    fn test_pick_amount() {
        let pick1_amount = pick_em(50, 5).len();
        assert_eq!(pick1_amount, 5);

        let pick2_amount = pick_em(50, 1).len();
        assert_eq!(pick2_amount, 1);
    }

    #[test]
    #[should_panic]
    // This situation is not possible, so it should panic
    fn test_pick_amount_invalid() {
        pick_em(50, 0);
    }

    #[test]
    // Test the minimum pick
    fn test_pick() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(numbers, pick_em(5, 5));
    }

    #[test]
    #[should_panic]
    // This should break pick_em()
    fn test_pick_invalid() {
        pick_em(1, 5);
    }

    #[test]
    // Make sure the print out looks ok
    fn test_printout() {
        let winners = Winners {
            winning_numbers: vec![1, 2, 3, 4, 5],
            bonus_numbers: vec![1],
        };

        let output = print_winners(winners);
        assert!(output.contains("1-2-3-4-5 (1)"));
    }

    #[test]
    #[should_panic]
    // What will an empty vector do?
    fn test_printout_invalid() {
        let winners = Winners {
            winning_numbers: Vec::new(),
            bonus_numbers: Vec::new(),
        };

        print_winners(winners);
    }

}

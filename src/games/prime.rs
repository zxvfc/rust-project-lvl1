use crate::{
    engine::{GameData, QuestionAndAnswer},
    utils::get_rand,
};

const RULES: &str = "Answer 'yes' if given number is prime. Otherwise answer 'no'.";

pub fn get_data() -> GameData {
    let questions_and_answers = std::array::from_fn(|_| {
        let num = get_rand();
        QuestionAndAnswer {
            question: num.to_string(),
            answer: if is_prime(num) { "yes" } else { "no" }.to_string(),
        }
    });
    GameData {
        description: RULES.to_string(),
        questions_and_answers,
    }
}

fn is_prime(num: u32) -> bool {
    match num {
        0 | 1 => false,
        2 | 3 => true,
        n if n % 2 == 0 || n % 3 == 0 => false,
        n => {
            let mut i = 5;
            while i * i <= n {
                if n % i == 0 || n % (i + 2) == 0 {
                    return false;
                }
                i += 6;
            }
            true
        }
    }
}

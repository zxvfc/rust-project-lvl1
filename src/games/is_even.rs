use crate::constants::GameData;
use crate::utils;

const RULES: &str = "Answer 'yes' if the number is even, otherwise answer 'no'.";

pub fn get_data() -> GameData {
    let questions_and_answers = std::array::from_fn(|_| {
        let number = utils::get_rand();
        let correct_answer = if number % 2 == 0 { "yes" } else { "no" }.to_string();
        let question = number.to_string();
        (question, correct_answer)
    });
    (RULES.to_string(), questions_and_answers)
}

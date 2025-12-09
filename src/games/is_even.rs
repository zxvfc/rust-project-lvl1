use crate::{
    engine::{GameData, QuestionAndAnswer},
    utils,
};

const RULES: &str = "Answer 'yes' if the number is even, otherwise answer 'no'.";

pub fn get_data() -> GameData {
    let questions_and_answers = std::array::from_fn(|_| {
        let number = utils::get_rand();
        QuestionAndAnswer {
            question: number.to_string(),
            answer: if number % 2 == 0 { "yes" } else { "no" }.to_string(),
        }
    });
    GameData {
        description: RULES.to_string(),
        questions_and_answers,
    }
}

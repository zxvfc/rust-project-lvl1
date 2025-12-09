use crate::{
    engine::{GameData, QuestionAndAnswer},
    utils,
};

type MathOperation = (char, fn(u32, u32) -> i32);

const SUPPORTED_OPERATIONS: [MathOperation; 3] = [
    ('+', |a, b| (a + b) as i32),
    ('-', |a, b| (a - b) as i32),
    ('*', |a, b| (a * b) as i32),
];

const RULES: &str = "What is the result of the expression?";

pub fn get_data() -> GameData {
    let questions_and_answers = std::array::from_fn(|_| {
        let operation_index =
            (utils::get_rand_in_range(0..SUPPORTED_OPERATIONS.len() as u32)) as usize;
        let (sign, operation) = SUPPORTED_OPERATIONS[operation_index];

        let num1 = utils::get_rand();
        let num2 = utils::get_rand();

        QuestionAndAnswer {
            question: format!("{num1} {sign} {num2}"),
            answer: operation(num1, num2).to_string(),
        }
    });
    GameData {
        description: RULES.to_string(),
        questions_and_answers,
    }
}

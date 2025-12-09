use crate::{
    engine::{GameData, QuestionAndAnswer},
    utils::{get_rand, get_rand_in_range},
};
use std::ops::RangeInclusive;

const RULES: &str = "What number is missing in the progression?";
const PROGRESSION_LENGTH: usize = 10;
const PROGRESSION_STEP_RANGE: RangeInclusive<u32> = 2..=5;

pub fn get_data() -> GameData {
    let questions_and_answers = std::array::from_fn(|_| {
        let seed = get_rand();
        let step = get_rand_in_range(PROGRESSION_STEP_RANGE);
        let progression = generate_progression(seed, step);

        let hidden_item_index = get_rand_in_range(1..PROGRESSION_LENGTH as u32) as usize;
        QuestionAndAnswer {
            question: build_question(progression, hidden_item_index),
            answer: progression[hidden_item_index as usize].to_string(),
        }
    });
    GameData {
        description: RULES.to_string(),
        questions_and_answers,
    }
}

fn generate_progression(seed: u32, step: u32) -> [u32; PROGRESSION_LENGTH] {
    let mut progression = [0; PROGRESSION_LENGTH];
    progression[0] = seed;
    for i in 1..PROGRESSION_LENGTH {
        progression[i] = progression[i - 1] + step;
    }
    progression
}

fn build_question(progression: [u32; PROGRESSION_LENGTH], item_to_hide: usize) -> String {
    progression
        .iter()
        .enumerate()
        .map(|(i, &num)| {
            if i == item_to_hide {
                "..".to_string()
            } else {
                num.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

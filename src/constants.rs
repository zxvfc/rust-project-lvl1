use std::ops::RangeInclusive;

pub const RAND_RANGE: RangeInclusive<u32> = 0..=100;
pub const GAME_ROUNDS: usize = 3;

pub type QuestionAndAnswer = (String, String);
pub type QuestionsAndAnswers = [QuestionAndAnswer; GAME_ROUNDS];
pub type GameData = (String, QuestionsAndAnswers);

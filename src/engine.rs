use crate::{constants, utils::ask};

pub struct QuestionAndAnswer {
    pub(crate) question: String,
    pub(crate) answer: String,
}

pub struct GameData {
    pub(crate) description: String,
    pub(crate) questions_and_answers: [QuestionAndAnswer; constants::GAME_ROUNDS],
}

pub fn run_game(
    user_name: &str,
    GameData {
        description,
        questions_and_answers,
    }: &GameData,
) {
    println!("{}", description);
    for QuestionAndAnswer { question, answer } in questions_and_answers {
        println!("Question: {question}");
        let user_answer = ask("Your answer: ");
        let user_answer = user_answer.trim().to_lowercase();
        if answer != &user_answer {
            println!("'{user_answer}' is wrong answer ;(. Correct answer was '{answer}'");
            println!("\nLet's try again, {user_name}!");
            return;
        }
        println!("Correct!");
    }
    println!("\nCongratulations, {user_name}!")
}

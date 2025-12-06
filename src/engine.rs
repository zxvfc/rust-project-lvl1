use crate::{constants::QuestionsAndAnswers, utils::ask};

pub fn run_game(
    user_name: &str,
    game_description: String,
    questions_and_answers: &QuestionsAndAnswers,
) {
    println!("{}", game_description);
    for (question, correct_answer) in questions_and_answers {
        println!("Question: {question}");
        let user_answer = ask("Your answer: ");
        let user_answer = user_answer.trim().to_lowercase();
        if *correct_answer != user_answer {
            println!("'{user_answer}' is wrong answer ;(. Correct answer was '{correct_answer}' ");
            println!("\nLet's try again, {user_name}!");
            return;
        }
        println!("Correct!");
    }
    println!("\nCongratulations, {user_name}!")
}

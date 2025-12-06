use crate::{constants::GameData, engine::run_game};

mod constants;
mod engine;
mod games;
mod utils;

type GAME = (&'static str, fn() -> GameData);
const GAMES: [GAME; 5] = [
    ("Is Even", || games::is_even::get_data()),
    ("Calc", || games::calc::get_data()),
    ("GCD", || games::gcd::get_data()),
    ("Progression", || games::progression::get_data()),
    ("Prime", || games::prime::get_data()),
];

fn main() {
    println!("Welcome to the Brain Games!");

    let name = utils::ask("My I ask your name?\n: ");
    let user_name = name.trim();

    println!("Hello, {user_name}");
    loop {
        let input = menu();
        let selection: usize = match input.trim().parse() {
            Err(_) => continue,
            Ok(num) => {
                if num == 0 {
                    return;
                }
                if num > GAMES.len() {
                    continue;
                }
                num
            }
        };
        let (_, get_game_data) = GAMES[selection - 1];
        let (game_description, questions_and_answers) = get_game_data();
        run_game(user_name, game_description, &questions_and_answers);
        println!();
    }
}

fn menu() -> String {
    println!("Please, enter a game number and press 'Enter'.");
    for (i, (name, _)) in GAMES.iter().enumerate() {
        println!("{} - {name}", i + 1);
    }
    println!("0 - Exit");
    utils::ask(": ")
}

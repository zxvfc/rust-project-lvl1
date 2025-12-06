use std::cmp;

use crate::{constants::GameData, utils::get_rand};

const RULES: &str = "Find the greatest common divisor of given numbers.";

pub fn get_data() -> GameData {
    let questions_and_answers = std::array::from_fn(|_| {
        let num1 = get_rand() as i32;
        let num2 = get_rand() as i32;
        let answer = find_gcd(num1, num2);
        (format!("{num1} {num2}"), answer.to_string())
    });
    (RULES.to_string(), questions_and_answers)
}

fn find_gcd(num1: i32, num2: i32) -> i32 {
    if num2 == 0 {
        num1
    } else {
        find_gcd(num2, num1 % num2)
    }
}

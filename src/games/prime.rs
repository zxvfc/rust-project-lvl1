use crate::{constants::GameData, utils::get_rand};

const RULES: &str = "Answer 'yes' if given number is prime. Otherwise answer 'no'.";

pub fn get_data() -> GameData {
    let questions_and_answers = std::array::from_fn(|_| {
        let num = get_rand();
        let answer = if is_prime(num as f64) { "yes" } else { "no" };
        (num.to_string(), answer.to_string())
    });
    (RULES.to_string(), questions_and_answers)
}

fn is_prime(num: f64) -> bool {
    if num == 2.0 || num == 3.0 {
        return true;
    }
    if num <= 1.0 || num % 2.0 == 0.0 || num % 3.0 == 0.0 {
        return false;
    }
    let mut i = 5.0;
    let sqrt = (num as f64).sqrt();
    while i <= sqrt {
        if num % i == 0.0 || num % (i + 2.0) == 0.0 {
            return false;
        }
        i += 6.0;
    }
    true
}

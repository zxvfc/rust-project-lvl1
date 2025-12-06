use rand::{Rng, distributions::uniform::SampleRange};
use std::io::{self, Write};

use crate::constants;

pub fn ask(question: &str) -> String {
    let mut input = String::new();
    while input.trim().is_empty() {
        print!("{}", question);
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
    input
}

pub fn get_rand() -> u32 {
    get_rand_in_range(constants::RAND_RANGE)
}

pub fn get_rand_in_range<RangeType>(range: RangeType) -> u32
where
    RangeType: SampleRange<u32>,
{
    rand::thread_rng().gen_range(range)
}

extern crate rand;

use rand::Rng;
use rand::seq::SliceRandom;

pub fn gen_random_range(low: i64, high: i64) -> Result<i64, &'static str> {
    let mut rng = rand::thread_rng();
    if low >= high {
        Err("low mustn't be larger than high")
    } else {
        Ok(rng.gen_range(low, high))
    }
}

pub fn select_random(options: &Vec<String>) -> Result<String, &'static str> {
    let mut rng = rand::thread_rng();
    match options.choose(&mut rng) {
        Some(option) => Ok(option.to_string()),
        None => Err("Options length must larger than 0"),
    }
}

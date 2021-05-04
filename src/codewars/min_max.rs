#![allow(dead_code)]

use itertools::Itertools;
use itertools::MinMaxResult;
use std::num::ParseIntError;

fn parse_input(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split_whitespace().map(|s| s.parse::<i64>()).collect()
}

fn min_max(numbers: &[i64]) -> Option<(i64, i64)> {
    match numbers.iter().copied().minmax() {
        MinMaxResult::MinMax(min, max) => Some((min, max)),
        MinMaxResult::OneElement(minmax) => Some((minmax, minmax)),
        MinMaxResult::NoElements => None,
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "1 2 3 -1337 42 4484 1231 -1231 123131";
    let numbers = parse_input(input)?;

    let res = match min_max(&numbers) {
        Some((min, max)) => format!("Min: {min}, Max: {max}", min = min, max = max),
        None => format!("Empty input"),
    };
    
    dbg!(res);

    Ok(())
}

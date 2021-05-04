#![allow(dead_code)]
use itertools::Itertools;

/*
Complete the solution so that it splits the string into pairs of two characters. If the string contains an odd number of characters then it should replace the missing second character of the final pair with an underscore ('_').

Examples:

solution("abcdef") // should return ["ab", "cd", "ef"]
solution("abcdefg") // should return ["ab", "cd", "ef", "g_"]
*/

fn optimized(s: &str) -> Vec<String> {
    s.chars()
        .chunks(2)
        .into_iter()
        .map(|c| c.pad_using(2, |_| '_').collect())
        .collect()
}

fn solution(s: &str) -> Vec<String> {
    (0..s.len())
        .filter(|index| index % 2 == 0)
        .map(|x| -> String {
            if x + 2 <= s.len() {
                return s[x..x + 2].to_string();
            }

            return s[x..=x].to_string() + "_";
        })
        .collect()
}

fn main() {
    dbg!(solution("abcdef"));
    dbg!(solution("abcdefg"));
}

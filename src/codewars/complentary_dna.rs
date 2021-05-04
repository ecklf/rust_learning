#![allow(dead_code)]

/*
Deoxyribonucleic acid (DNA) is a chemical found in the nucleus of cells and carries the "instructions" for the development and functioning of living organisms.

If you want to know more http://en.wikipedia.org/wiki/DNA

In DNA strings, symbols "A" and "T" are complements of each other, as "C" and "G". You have function with one side of the DNA (string, except for Haskell); you need to get the other complementary side. DNA strand is never empty or there is no DNA at all (again, except for Haskell).
*/

fn decode(s: &str) -> String {
    s.chars()
        .fold(String::with_capacity(s.len()), |mut acc, char| {
            acc.push(match char {
                'A' => 'T',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _ => char,
            });

            acc
        })
}

pub fn main() {
    dbg!(decode("ATTGC"));
}

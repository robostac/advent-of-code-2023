use scanner::Scanner;
use std::collections::*;
use std::io;

fn calc_score(s: &str) -> usize {
    let mut digits = Vec::new();
    for x in s.chars() {
        if let Some(v) = x.to_digit(10) {
            digits.push(v as usize);
        }
    }
    if digits.len() < 2 {
        return 0;
    }
    digits[0] * 10 + *digits.last().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    let mut part1 = 0;
    let mut part2 = 0;
    while let Some(s) = scan.next::<String>() {
        part1 += calc_score(&s);

        let mut p2s = String::new();
        let swaps = [
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
            ("zero", "0"),
        ];
        for (i, x) in s.chars().enumerate() {
            let mut found = false;
            for (src, dest) in swaps.iter() {
                if s[i..].starts_with(*src) {
                    p2s += *dest;
                    found = true
                }
            }
            if found == false {
                p2s.push(x);
            }
        }

        part2 += calc_score(&p2s);
    }

    println!("{}", part1);
    println!("{}", part2);
}

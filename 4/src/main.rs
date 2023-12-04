use std::collections::*;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut cards = Vec::new();

    let make_set = |x: &str| {
        x.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<HashSet<_>>()
    };

    for l in stdin.lock().lines() {
        let s = l.unwrap();
        if let Some((_, v)) = s.split_once(':') {
            let (win, have) = v.split_once('|').unwrap();

            let h = make_set(have);
            let w = make_set(win);

            let count = w.intersection(&h).count();

            cards.push(count);
        }
    }
    let mut counts = vec![1; cards.len()];

    let mut p1 = 0;
    for (i, x) in cards.iter().enumerate() {
        if *x > 0 {
            p1 += 1 << (x - 1);
            for j in (i + 1)..(cards.len().min(i + *x + 1)) {
                counts[j] += counts[i]
            }
        }
    }

    println!("{}", p1);
    println!("{:?}", counts.iter().sum::<usize>());
}

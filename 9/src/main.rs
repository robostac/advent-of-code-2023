use std::io;
use std::io::BufRead;

fn solve_val(x: &Vec<i64>) -> i64 {
    let mut test = vec![x.clone()];
    while test.last().unwrap().len() != 1 {
        let z = test
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, x)| x - test.last().unwrap()[i - 1])
            .collect();
        test.push(z)
    }

    for i in (0..test.len()).rev().skip(1) {
        let v = test[i].last().unwrap() + test[i + 1].last().unwrap();
        test[i].push(v);
    }
    *test[0].last().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let mut p1 = 0;
    let mut p2 = 0;
    for l in stdin.lock().lines().map(|x| x.unwrap()) {
        let mut p = l
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        p1 += solve_val(&p);
        p.reverse();
        p2 += solve_val(&p);
    }
    println!("{}", p1);
    println!("{}", p2);
}

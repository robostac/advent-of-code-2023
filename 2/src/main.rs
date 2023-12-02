use scanner::Scanner;
use std::collections::*;
use std::io;
fn main() {
    let stdin = io::stdin();
    let mut scan = Scanner::new(stdin.lock());
    let mut part1 = 0;
    let mut last = String::new();
    let mut cg = 0;
    let mut tg = 0;
    let mut badgames = HashSet::new();
    let mut mins = [0, 0, 0];
    let mut p2 = 0;
    while let Some(mut s) = scan.next::<String>() {
        s = s.replace(",", "");
        s = s.replace(";", "");
        s = s.replace(":", "");

        if last == "Game" {
            cg = s.parse::<usize>().ok().unwrap();
            tg += cg;
            p2 += mins[0] * mins[1] * mins[2];
            mins = [0, 0, 0];
        } else {
            match s.as_str() {
                "red" => {
                    let v = last.parse::<usize>().ok().unwrap();
                    if v > 12 {
                        badgames.insert(cg);
                    }
                    mins[0] = mins[0].max(v);
                }
                "blue" => {
                    let v = last.parse::<usize>().ok().unwrap();
                    if v > 14 {
                        badgames.insert(cg);
                    }
                    mins[1] = mins[1].max(v);
                }
                "green" => {
                    let v = last.parse::<usize>().ok().unwrap();
                    if v > 13 {
                        badgames.insert(cg);
                    }
                    mins[2] = mins[2].max(v);
                }
                _ => {}
            }
        }
        last = s;
    }
    p2 += mins[0] * mins[1] * mins[2];
    println!("{}", tg - badgames.iter().sum::<usize>());
    println!("{}", p2);
}

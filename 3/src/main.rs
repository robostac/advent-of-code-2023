use scanner::Scanner;
use std::collections::*;
use std::io;
fn main() {
    let stdin = io::stdin();
    let mut scan = Scanner::new(stdin.lock());
    let mut grid = HashMap::new();
    let mut symbols = HashMap::new();
    let mut y = 1i64;
    let mut mx = 0;
    let mut my = 0;
    while let Some(s) = scan.next::<String>() {
        for (x, v) in s.chars().enumerate() {
            let x = x as i64 + 1;
            if v == '.' {
                continue;
            }
            if let Some(c) = v.to_digit(10) {
                grid.insert((x, y), c);
            } else {
                symbols.insert((x, y), v);
            }
            mx = mx.max(x);
            my = my.max(y);
        }
        y += 1;
    }

    let mut ans = 0;
    let mut gears = HashMap::new();
    for y in 1..=my {
        let mut value = 0;
        let mut symbol = false;
        let mut syms = HashSet::new();
        for x in 1..=(mx + 1) {
            if let Some(p) = grid.get(&(x, y)) {
                value *= 10;
                value += p;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        symbol |= symbols.contains_key(&(x + dx, y + dy));
                        syms.insert((x + dx, y + dy));
                    }
                }
            } else {
                if value > 0 {
                    if symbol {
                        ans += value;
                    }
                    for x in syms {
                        gears.entry(x).or_insert(Vec::new()).push(value);
                    }
                    syms = HashSet::new();
                    value = 0;
                    symbol = false;
                }
            }
        }
    }
    let mut p2 = 0;
    for (x, s) in symbols {
        if s == '*' {
            if let Some(v) = gears.get(&x) {
                if v.len() == 2 {
                    p2 += v[0] * v[1];
                }
            }
        }
    }
    println!("{}", ans);
    println!("{:?}", p2);
}

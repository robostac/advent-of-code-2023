use std::collections::*;
use std::io;
use std::io::BufRead;

fn find_reflection(
    pattern: &HashMap<(i64, i64), bool>,
    ignore_row: i64,
    ignore_col: i64,
) -> Option<(i64, i64)> {
    let width = pattern.keys().map(|x| x.0).max().unwrap() as i64;
    let height = pattern.keys().map(|x| x.1).max().unwrap() as i64;
    let mut cols = (1..=width).collect::<HashSet<_>>();
    let mut rows = (1..=height).collect::<HashSet<_>>();
    cols.retain(|x| *x != ignore_col);
    rows.retain(|x| *x != ignore_row);
    for y in 0..=height {
        cols.retain(|&x| {
            pattern.iter().filter(|(k, _)| k.1 == y).all(|(p, v)| {
                let dx = x - p.0;
                let opposite = (x + dx - 1, p.1);
                *pattern.get(&opposite).unwrap_or(v) == *v
            })
        })
    }
    for x in 0..=width {
        rows.retain(|&y| {
            pattern.iter().filter(|(k, _)| k.0 == x).all(|(p, v)| {
                let dy = y - p.1;
                let opposite = (x, y + dy - 1);
                *pattern.get(&opposite).unwrap_or(v) == *v
            })
        })
    }

    if let Some(p) = cols.iter().next() {
        return Some((*p, 1));
    } else if let Some(p) = rows.iter().next() {
        return Some((*p, 100));
    } else {
        return None;
    }
}
fn main() {
    let stdin = io::stdin();
    let mut patterns = Vec::new();
    let mut current = HashMap::new();
    let mut sy = 0;
    for (y, l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        if l.trim().len() == 0 {
            patterns.push(current);
            current = HashMap::new();
            sy = y + 1;
        } else {
            for (x, v) in l.chars().enumerate() {
                current.insert((x as i64, (y - sy) as i64), v == '#');
            }
        }
    }
    patterns.push(current);

    let mut p1 = 0;
    let mut p2 = 0;
    for (_pnum, pattern) in patterns.iter_mut().enumerate() {
        if let Some(p1ref) = find_reflection(&pattern, 0, 0) {
            p1 += p1ref.0 * p1ref.1;
            let ignore_col;
            let ignore_row;
            if p1ref.1 == 1 {
                ignore_col = p1ref.0;
                ignore_row = 0;
            } else {
                ignore_col = 0;
                ignore_row = p1ref.0;
            }
            let points = pattern.keys().cloned().collect::<Vec<_>>();
            for p in points {
                let wall = pattern[&p];
                pattern.insert(p, !wall);
                let ans = find_reflection(&pattern, ignore_row, ignore_col);
                pattern.insert(p, wall);
                if let Some(v) = ans {
                    p2 += v.0 * v.1;
                    break;
                }
            }
        }
    }
    println!("{:?}", p1);
    println!("{:?}", p2);
}

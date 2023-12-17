use std::collections::*;
use std::io;
use std::io::BufRead;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
    fn add(&self, other: &Point) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut mx = 0;
    let mut my = 0;
    let mut grid = HashMap::new();
    for (y, l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        for (x, c) in l.chars().enumerate() {
            grid.insert(
                Point::new(x as i64, y as i64),
                c.to_digit(10).unwrap() as i64,
            );
            mx = (x as i64).max(mx);
        }
        my = y as i64;
    }
    let solve = |min_consec: i64, max_consec: i64| {
        let mut queue = VecDeque::new();
        let mut best = HashMap::new();
        queue.push_back((Point::new(0, 0), Point::new(1, 0), 1, 0i64));
        queue.push_back((Point::new(0, 0), Point::new(0, 1), 1, 0i64));
        let mut least_loss = i64::MAX;
        while let Some((p, d, c, v)) = queue.pop_front() {
            if c >= min_consec {
                if p == Point::new(mx, my) {
                    least_loss = least_loss.min(v);
                }
            }
            if v >= least_loss {
                continue;
            }
            if let Some(bv) = best.get(&(p, d, c)) {
                if *bv < v {
                    continue;
                }
            }
            let mut directions = Vec::new();
            if c >= min_consec {
                directions = vec![(Point::new(d.y, d.x), 1), (Point::new(-d.y, -d.x), 1)];
            }
            if c < max_consec {
                directions.push((d, c + 1));
            }
            for (nd, nc) in directions {
                let np = p.add(&nd);
                // println!("{:?}", np);
                if let Some(&value) = grid.get(&np) {
                    let nv = v + value;
                    let e = best.entry((np, nd, nc)).or_insert(i64::MAX);
                    if *e <= nv {
                        continue;
                    }
                    *e = nv;
                    queue.push_back((np, nd, nc, nv));
                }
            }
        }
        least_loss
    };
    let p1ans = solve(0, 3);
    println!("{}", p1ans);
    let p2ans = solve(4, 10);
    println!("{}", p2ans);
}

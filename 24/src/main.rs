use std::collections::*;
use std::io;
use std::io::BufRead;

// fn lcm(fi

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Point { x, y, z }
    }
    fn add(&self, other: &Point) -> Self {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Hail {
    p: Point,
    v: Point,
}

impl Hail {
    fn new(mut inp: String, id: usize) -> Self {
        inp = inp.replace("@", " ");
        inp = inp.replace(",", " ");
        println!("{}", inp);
        let points = inp
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mut b = Hail {
            p: Point::new(points[0], points[1], points[2]),
            v: Point::new(points[3], points[4], points[5]),
        };
        b
    }

    fn equation(&self) -> (f64, f64) {
        let next = self.p.add(&self.v);
        let slope = (next.y - self.p.y) as f64 / (next.x - self.p.x) as f64;
        let intersect = self.p.y as f64 - slope * self.p.x as f64;
        (slope, intersect)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut hail = Vec::new();

    for (_y, l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        hail.push(Hail::new(l, _y));
    }
    let minx = 200000000000000.0; //7.0;
    let maxx = 400000000000000.0; //27.0;
    let mut count = 0;

    for (i, h1) in hail.iter().enumerate() {
        let (m1, c1) = h1.equation();
        for (j, h2) in hail.iter().enumerate().skip(i + 1) {
            let (m2, c2) = h2.equation();
            let px = (c2 - c1) / (m1 - m2);
            let py = m1 * px + c1;
            let tx = (px - h1.p.x as f64).signum() == h1.v.x.signum() as f64;
            let t2 = (px - h2.p.x as f64).signum() == h2.v.x.signum() as f64;
            if tx && t2 {
                if px >= minx && px <= maxx && py >= minx && py <= maxx {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}

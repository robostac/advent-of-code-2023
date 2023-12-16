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

fn move_beam(
    point: Point,
    direction: Point,
    seen: &mut HashSet<(Point, Point)>,
    grid: &HashMap<Point, char>,
) {
    if let Some(&c) = grid.get(&point) {
        if seen.insert((point, direction)) == false {
            return;
        }
        match c {
            '.' => move_beam(point.add(&direction), direction, seen, grid),
            '-' => {
                if direction.x != 0 {
                    move_beam(point.add(&direction), direction, seen, grid);
                } else {
                    for d in [Point::new(1, 0), Point::new(-1, 0)] {
                        move_beam(point.add(&d), d, seen, grid);
                    }
                }
            }
            '|' => {
                if direction.y != 0 {
                    move_beam(point.add(&direction), direction, seen, grid);
                } else {
                    for d in [Point::new(0, 1), Point::new(0, -1)] {
                        move_beam(point.add(&d), d, seen, grid);
                    }
                }
            }
            '/' => {
                let d = Point::new(-direction.y, -direction.x);
                move_beam(point.add(&d), d, seen, grid);
            }
            '\\' => {
                let d = Point::new(direction.y, direction.x);
                move_beam(point.add(&d), d, seen, grid);
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut mx = 0;
    let mut my = 0;
    let mut grid = HashMap::new();
    for (y, l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        for (x, c) in l.chars().enumerate() {
            grid.insert(Point::new(x as i64, y as i64), c);
            mx = (x as i64).max(mx);
        }
        my = y as i64;
    }

    let solve = |p: Point, d: Point| {
        let mut seen = HashSet::new();
        move_beam(p, d, &mut seen, &grid);
        let energised = seen.iter().map(|x| x.0.clone()).collect::<HashSet<_>>();
        return energised.len();
    };

    println!("{:?}", solve(Point::new(0, 0), Point::new(1, 0)));

    let mut best = 0;
    for x in 0..=mx {
        best = best.max(solve(Point::new(x, 0), Point::new(0, 1)));
        best = best.max(solve(Point::new(x, my), Point::new(0, -1)));
        // best =
    }

    for y in 0..=my {
        best = best.max(solve(Point::new(0, y), Point::new(1, 0)));
        best = best.max(solve(Point::new(mx, y), Point::new(-1, 0)));
    }
    println!("{:?}", best);
}

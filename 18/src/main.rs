use std::collections::*;
use std::io;
use std::io::BufRead;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    const fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    fn add_mult(&self, other: &Point, count: i64) -> Self {
        Point::new(self.x + other.x * count, self.y + other.y * count)
    }
}

fn line(dc: char, count: i64, origin: Point) -> (Point, Point, char) {
    let d = match dc {
        'R' => Point::new(1, 0),
        'L' => Point::new(-1, 0),
        'D' => Point::new(0, 1),
        'U' => Point::new(0, -1),
        _ => unreachable!(),
    };

    (origin, origin.add_mult(&d, count), dc)
}

const UP: usize = 1 << 0;
const DOWN: usize = 1 << 1;
const LEFT: usize = 1 << 2;
const RIGHT: usize = 1 << 3;

fn main() {
    let stdin = io::stdin();

    let mut p1start = Point::new(0, 0);
    let mut p1lines = Vec::new();
    let mut p2start = Point::new(0, 0);
    let mut p2lines = Vec::new();
    for (_y, l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        let inst = l.split_ascii_whitespace().collect::<Vec<_>>();

        let count = inst[1].parse::<i64>().unwrap();
        p1lines.push(line(inst[0].chars().next().unwrap(), count, p1start));
        p1start = p1lines.last().unwrap().1;

        let p2count = i64::from_str_radix(&inst[2][2..7], 16).unwrap();
        let p2d = ['R', 'D', 'L', 'U'][inst[2][7..=7].parse::<usize>().unwrap()];
        p2lines.push(line(p2d, p2count, p2start));
        p2start = p2lines.last().unwrap().1;
    }

    let solve = |x: &mut Vec<(Point, Point, char)>| {
        let mut ans = 0;
        let mut sig_y = BTreeSet::new();
        for p in x.iter() {
            for z in -1..=1 {
                sig_y.insert(p.0.y + z);
                sig_y.insert(p.1.y + z);
            }

            let lenx = (p.0.x - p.1.x).abs();
            let leny = (p.1.y - p.0.y).abs();
            ans += lenx.max(leny);
        }

        x.sort_by_key(|x| x.0.x.min(x.1.x));

        let mut last_count = 0;
        let mut last_y = *sig_y.iter().next().unwrap();
        for y in sig_y {
            ans += last_count * (y - last_y);
            last_count = 0;
            let mut corners = BTreeMap::new();
            for p in x.iter() {
                let horiz = p.0.y == p.1.y;
                let miny = p.0.y.min(p.1.y);
                let maxy = p.0.y.max(p.1.y);
                if miny <= y && maxy >= y {
                    if horiz {
                        *corners.entry(p.0.x.min(p.1.x)).or_insert(0) |= RIGHT;
                        *corners.entry(p.0.x.max(p.1.x)).or_insert(0) |= LEFT;
                    } else {
                        let e = corners.entry(p.0.x).or_insert(0);
                        if miny < y {
                            *e |= UP;
                        }
                        if maxy > y {
                            *e |= DOWN;
                        }
                    }
                }
            }
            if corners.len() == 0 {
                continue;
            }
            let mut last_bend = 0;
            let mut last_inside = false;
            let mut last_entry = *corners.iter().next().unwrap().0;
            let mut inside = false;

            for x in corners.clone() {
                let mut ni = inside;
                if x.1 == UP | DOWN {
                    ni = !inside;
                }
                if x.1 & RIGHT > 0 {
                    last_bend = x.1;
                    last_inside = inside;
                    ni = false;
                } else if x.1 & LEFT > 0 {
                    if (last_bend & (UP | DOWN)) != (x.1 & (UP | DOWN)) {
                        ni = !last_inside;
                    } else {
                        ni = last_inside;
                    }
                }

                if inside != ni {
                    if ni {
                        last_entry = x.0 + 1;
                    } else {
                        last_count += x.0 - last_entry;
                    }
                    inside = ni;
                }
            }
            last_y = y;
        }
        ans
    };

    println!("{}", solve(&mut p1lines));
    println!("{}", solve(&mut p2lines));
}

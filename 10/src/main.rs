use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

const UP: usize = 1 << 0;
const DOWN: usize = 1 << 1;
const LEFT: usize = 1 << 2;
const RIGHT: usize = 1 << 3;
const START: usize = 1 << 4;

fn opposite(d: usize) -> usize {
    match d {
        UP => DOWN,
        DOWN => UP,
        LEFT => RIGHT,
        RIGHT => LEFT,
        x => x,
    }
}

fn move_direction(p: (i64, i64), d: usize) -> (i64, i64) {
    let diff = match d {
        UP => (0, -1),
        DOWN => (0, 1),
        LEFT => (-1, 0),
        RIGHT => (1, 0),
        _ => panic!(),
    };
    (p.0 + diff.0, p.1 + diff.1)
}

fn get_next(v: usize, d: usize) -> Option<usize> {
    let d = opposite(d);
    if v & d == 0 {
        return None;
    }
    Some(v & !d)
}

fn main() {
    let stdin = io::stdin();
    let mut g = HashMap::new();
    let mut start = (0, 0);
    for (y, l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        for (i, v) in l.chars().enumerate() {
            let y = y as i64;
            let i = i as i64;
            let v = match v {
                '|' => UP | DOWN,
                '-' => LEFT | RIGHT,
                'L' => UP | RIGHT,
                'J' => UP | LEFT,
                '7' => DOWN | LEFT,
                'F' => DOWN | RIGHT,
                'S' => START,
                _ => 0,
            };
            if v & START > 0 {
                start = (i, y);
            }
            g.insert((i, y), v);
        }
    }
    let mut sval = 0;

    let follow = |mut p: (i64, i64), mut d: usize| {
        let mut route = Vec::new();

        loop {
            if let Some(v) = g.get(&p) {
                if v & START > 0 {
                    return route;
                }
                route.push(p);
                if let Some(z) = get_next(*v, d) {
                    d = z;
                    p = move_direction(p, d);
                } else {
                    return Vec::new();
                }
            } else {
                return Vec::new();
            }
        }
    };

    for d in [LEFT, RIGHT, UP, DOWN] {
        let mut r = follow(move_direction(start, d), d);
        if r.len() > 0 {
            sval |= d;
            if sval.count_ones() == 2 {
                println!("{}", r.len() / 2);
                r.push(start);
                g.insert(start, sval);
                let r = r.into_iter().collect::<HashSet<_>>();
                let minx = r.iter().min_by_key(|x| x.0).unwrap().0;
                let maxx = r.iter().max_by_key(|x| x.0).unwrap().0;
                let miny = r.iter().min_by_key(|x| x.1).unwrap().1;
                let maxy = r.iter().max_by_key(|x| x.1).unwrap().1;
                let mut enclosed = 0;
                for y in miny..=maxy {
                    let mut count = 0;
                    let mut last_bend = 0;
                    for x in minx..=maxx {
                        if r.contains(&(x, y)) {
                            if let Some(&p) = g.get(&(x, y)) {
                                if p == LEFT | RIGHT {
                                } else if p == UP | DOWN {
                                    count ^= 1;
                                } else if p & RIGHT > 0 {
                                    last_bend = get_next(p, LEFT).unwrap();
                                } else {
                                    if opposite(last_bend) & p > 0 {
                                        count ^= 1;
                                    }
                                }
                            }
                        } else if count > 0 {
                            enclosed += 1;
                        }
                    }
                }

                println!("{}", enclosed);
                break;
            }
        }
    }
}

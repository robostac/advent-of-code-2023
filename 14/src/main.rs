use std::collections::*;
use std::io;
use std::io::BufRead;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Cell {
    Rock,
    Wall,
    Empty,
}

fn cycle(
    input: &HashMap<(i64, i64), Cell>,
    direction: (i64, i64),
    order: &Vec<(i64, i64)>,
) -> HashMap<(i64, i64), Cell> {
    let mut p1grid = input.clone();

    for &(x, y) in order.iter() {
        if let Some(Cell::Rock) = input.get(&(x, y)) {
            let mut cx = x;
            let mut cy = y;
            loop {
                let nx = cx + direction.0;
                let ny = cy + direction.1;
                if let Some(Cell::Empty) = p1grid.get(&(nx, ny)) {
                    cx = nx;
                    cy = ny;
                } else {
                    break;
                }
            }
            p1grid.insert((x, y), Cell::Empty);
            p1grid.insert((cx, cy), Cell::Rock);
        }
    }
    p1grid
}

fn main() {
    let stdin = io::stdin();
    let mut mx = 0;
    let mut my = 0;
    let mut grid = HashMap::new();
    for (y, l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        for (x, c) in l.chars().enumerate() {
            let v = match c {
                'O' => Cell::Rock,
                '#' => Cell::Wall,
                _ => Cell::Empty,
            };
            grid.insert((x as i64, y as i64), v);
            mx = (x as i64).max(mx);
        }
        my = y as i64;
    }
    let direction = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let mut order = vec![Vec::new(); 4];

    //north
    for y in 0..=my {
        for x in 0..=mx {
            order[0].push((x, y));
        }
    }
    //west
    for x in 0..=mx {
        for y in 0..=my {
            order[1].push((x, y));
        }
    }
    //south
    for y in (0..=my).rev() {
        for x in 0..=mx {
            order[2].push((x, y));
        }
    }
    //east
    for x in (0..=mx).rev() {
        for y in 0..=my {
            order[3].push((x, y));
        }
    }

    let grid = grid;
    let p1grid = cycle(&grid, direction[0], &order[0]);

    let p1 = p1grid
        .iter()
        .filter(|x| *x.1 == Cell::Rock)
        .map(|((_, y), _)| my - y + 1)
        .sum::<i64>();

    let mut p2grid_a = grid.clone();
    let mut p2grid_b = grid.clone();
    let mut curcycle_a = 0;
    let mut curcycle_b = 0;
    let tgt = 1000000000;
    let mut skip = false;
    while curcycle_a != tgt {
        for (i, d) in direction.iter().enumerate() {
            p2grid_a = cycle(&p2grid_a, *d, &order[i]);
        }
        curcycle_a += 1;
        if skip == false {
            for _ in 0..2 {
                for (i, d) in direction.iter().enumerate() {
                    p2grid_b = cycle(&p2grid_b, *d, &order[i]);
                }
                curcycle_b += 2;
            }
            if p2grid_a == p2grid_b {
                let cl = curcycle_b - curcycle_a;
                let skip_count = (tgt - curcycle_a) / cl;
                curcycle_a += skip_count * cl;
                skip = true;
            }
        }
    }

    let p2 = p2grid_a
        .iter()
        .filter(|x| *x.1 == Cell::Rock)
        .map(|((_, y), _)| my - y + 1)
        .sum::<i64>();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

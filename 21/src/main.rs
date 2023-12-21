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

fn binary_search_range_min(l: i64, r: i64, res: &dyn Fn(i64) -> bool) -> i64 {
    let mut l = l;
    let mut r = r;
    if res(l) {
        return l;
    }
    while l + 1 < r {
        let m = l.saturating_add(r) / 2;
        let v = res(m);
        if v == false {
            l = m;
        } else {
            r = m;
        }
    }
    return r;
}

fn binary_search_range_max(l: i64, r: i64, res: &dyn Fn(i64) -> bool) -> i64 {
    let mut l = l;
    let mut r = r;
    if res(r) {
        return r;
    }
    while (l + 1) < r {
        let m = l.saturating_add(r) / 2;
        let v = res(m);
        if v == true {
            l = m;
        } else {
            r = m;
        }
    }
    return l;
}

fn main() {
    let stdin = io::stdin();
    let mut mx = 0;
    let mut my = 0;
    let mut grid = HashSet::new();
    let mut start = Point::new(0, 0);
    for (y, l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '.' {
                grid.insert(Point::new(x as i64, y as i64));
            }
            if c == 'S' {
                grid.insert(Point::new(x as i64, y as i64));
                start = Point::new(x as i64, y as i64);
            }
            mx = (x as i64).max(mx);
        }
        my = y as i64;
    }
    let width = mx + 1;
    let height = my + 1;

    let calc_dist =
        |origin: Point, dist: &mut HashMap<(Point, Point), i64>, grid: &HashSet<Point>| {
            let mut queue = VecDeque::new();
            queue.push_back((origin, 0));
            let mut seen = HashSet::new();
            seen.insert(origin);
            while let Some((p, d)) = queue.pop_front() {
                dist.insert((origin, p), d);
                for direction in [
                    Point::new(0, 1),
                    Point::new(1, 0),
                    Point::new(-1, 0),
                    Point::new(0, -1),
                ] {
                    let np = p.add(&direction);
                    if grid.contains(&np) && seen.insert(np) {
                        queue.push_back((np, d + 1));
                    }
                }
            }
        };
    let modval = |x: i64, m: i64| ((x % m) + m) % m;

    let mut dist = HashMap::new();
    let top_left = Point::new(0, 0);
    let top_right = Point::new(mx, 0);
    let bottom_left = Point::new(0, my);
    let bottom_right = Point::new(mx, my);
    for p in [start, top_left, top_right, bottom_left, bottom_right] {
        calc_dist(p, &mut dist, &grid);
    }

    let mut p1 = 0;
    let steps = 64;
    for &p in grid.iter() {
        if let Some(&d) = dist.get(&(start, p)) {
            if d <= steps && d & 1 == steps & 1 {
                p1 += 1;
            }
        }
    }

    println!("{:?}", p1);

    let mut g2 = HashSet::new();
    for &x in grid.iter() {
        g2.insert(x);
        g2.insert(Point::new(x.x + width, x.y));
        g2.insert(Point::new(x.x + width, x.y + height));
        g2.insert(Point::new(x.x, x.y + height));
    }

    //need to have even lengths
    let grid = g2;
    let width = width * 2;
    let height = height * 2;

    //find any empty row / columns as these are potential routes
    let mut start_squares = HashSet::new();
    for x in 0..width {
        if (0..height).all(|y| grid.contains(&Point::new(x, y))) {
            start_squares.insert(Point::new(x, 0));
            start_squares.insert(Point::new(x, height - 1));
        }
    }
    for y in 0..height {
        if (0..width).all(|x| grid.contains(&Point::new(x, y))) {
            start_squares.insert(Point::new(0, y));
            start_squares.insert(Point::new(width - 1, y));
        }
    }

    calc_dist(start, &mut dist, &grid);
    for &p in start_squares.iter() {
        calc_dist(p, &mut dist, &grid);
    }

    let get_dist = |src: Point, dest: Point| {
        let row = modval(dest.y, height);
        let col = modval(dest.x, width);
        let sx = dest.x - col;
        let sy = dest.y - row;
        if let Some(dd) = dist.get(&(src, dest)) {
            return *dd;
        }
        let mut mindist = i64::MAX;
        for &sp in start_squares.iter() {
            for &ep in start_squares.iter() {
                if let Some(d1) = dist.get(&(src, sp)) {
                    if let Some(d2) = dist.get(&(ep, Point::new(col, row))) {
                        let mut nd = *d1 + *d2;
                        let target_point = Point::new(sx + ep.x, sy + ep.y);

                        nd += (target_point.x - sp.x).abs();
                        nd += (target_point.y - sp.y).abs();
                        mindist = mindist.min(nd);
                    }
                }
                // println!("{:?} {:?} {:?} {}", sp, ep, target_point, nd);
            }
        }
        return mindist;
    };

    let calc_square = |ep1: Point, ep2: Point, ep2dist: i64, steps: i64, parity: i64| {
        if steps < 0 {
            return 0;
        }
        let mut ans = 0;
        for &x in grid.iter() {
            let mut mindist = i64::MAX;
            if let Some(p) = dist.get(&(ep1, x)) {
                mindist = mindist.min(*p);
            }
            if let Some(p) = dist.get(&(ep2, x)) {
                mindist = mindist.min(*p + ep2dist);
            }
            if mindist <= steps && (mindist) % 2 == parity {
                ans += 1;
            }
        }
        ans
    };

    let steps = 26501365;
    // let steps = 50001;
    let min_square_y = (steps + 2 * height) / height;
    let min_square_x = (steps + 2 * width) / width;
    let calc = |sqr: Point| {
        let sqx = sqr.x;
        let sqy = sqr.y;
        let mut points = start_squares.iter().cloned().collect::<Vec<_>>();
        points.sort_by_key(|x| get_dist(start, x.add(&Point::new(sqx, sqy))));
        //check against two closest entry points as when we're on the axis these aren't equal distances
        let tgtp1 = points[0].add(&Point::new(sqx, sqy));
        let tgtp2 = points[1].add(&Point::new(sqx, sqy));
        let d1 = get_dist(start, tgtp1);
        let d2 = get_dist(start, tgtp2) - d1;
        calc_square(points[0], points[1], d2, steps - d1, (steps % 2) ^ (d1 % 2))
    };

    let solve_y = |y: i64| {
        let mut lx = binary_search_range_max(-min_square_x, 0, &|x| {
            calc(Point::new(x * width, y * height)) == 0
        });
        let mut rx = binary_search_range_min(0, min_square_x, &|x| {
            calc(Point::new(x * width, y * height)) == 0
        });
        let mut extra = 0;
        let max = calc(start);

        while lx <= rx {
            let sq = calc(Point::new(lx * width, y * height));
            extra += sq;
            lx += 1;
            if sq == max {
                break;
            }
        }
        while lx <= rx {
            let sq = calc(Point::new(rx * width, y * height));
            extra += sq;
            rx -= 1;

            if sq == max {
                break;
            }
        }
        extra += (rx - lx + 1) * max;

        (extra, (rx - lx + 1), max)
    };
    let mut p2 = 0;

    for y in -min_square_y..0 {
        let (mut ex, full, full_val) = solve_y(y);
        if full > 5 {
            for _yy in y..0 {
                // assert!(ex == solve_y(yy).0);
                p2 += ex;
                ex += 2 * full_val;
            }
            break;
        }
        p2 += ex;
    }

    for y in (1..min_square_y).rev() {
        let (mut ex, full, full_val) = solve_y(y);
        if full > 5 {
            for _yy in (1..=y).rev() {
                // assert!(ex == solve_y(yy).0);
                p2 += ex;
                ex += 2 * full_val;
            }
            break;
        }
        p2 += ex;
    }
    //0 is a weird special case

    p2 += solve_y(0).0;

    println!("{}", p2);
}

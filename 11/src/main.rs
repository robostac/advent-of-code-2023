use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut g = Vec::new();
    let mut maxx = 0;
    let mut maxy = 0;
    let mut xgal = HashSet::new();
    let mut ygal = HashSet::new();
    for (y, l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        for (i, v) in l.chars().enumerate() {
            maxx = maxx.max(i);
            let y = y as i64;
            let i = i as i64;

            if v == '#' {
                g.push((i, y));
                xgal.insert(i);
                ygal.insert(y);
            }
        }
        maxy = y;
    }

    let create_map = |occupied: &HashSet<i64>, max: usize, distance: i64| {
        let mut map = vec![0; max + 1];
        let mut n = 0;

        for i in 0..=max {
            map[i] = n;
            if occupied.contains(&(i as i64)) == false {
                n += distance - 1;
            }
            n += 1;
        }
        map
    };

    let solve = |g: &Vec<(i64, i64)>, distance: i64| {
        let mut g = g.clone();
        let xmap = create_map(&xgal, maxx, distance);
        let ymap = create_map(&ygal, maxy, distance);
        for (x, y) in g.iter_mut() {
            *x = xmap[*x as usize];
            *y = ymap[*y as usize];
        }
        let mut p1 = 0;
        for (i, g1) in g.iter().enumerate() {
            for g2 in g.iter().skip(i + 1) {
                let dist = (g1.0 - g2.0).abs() + (g1.1 - g2.1).abs();
                p1 += dist;
            }
        }
        println!("{:?}", p1);
    };
    solve(&g, 2);
    solve(&g, 1000000);
}

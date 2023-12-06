use scanner::Scanner;
use std::collections::*;
use std::io;
use std::option;

fn conv(x: &[i64]) -> i64 {
    let mut s = String::new();
    for p in x {
        s += &p.to_string();
    }
    s.parse::<i64>().unwrap()
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

//res must be increasing reach a peak and then start decreasing
//result will be the value corresponding to the peak
//returns a range that all need to be checked for max
fn ternary_search_range_max(l: i64, r: i64, res: &dyn Fn(i64) -> i64) -> (i64, i64) {
    let mut l = l;
    let mut r = r;

    loop {
        let len = (r - l + 1);
        if len <= 3 {
            break;
        }
        let step = len / 3;

        let m1 = l + step;
        let m2 = l + 2 * step;
        let v1 = res(m1);
        let v2 = res(m2);

        if v1 < v2 {
            l = m1;
        } else {
            r = m2;
        }
    }
    return (l, r);
}

fn is_possible(time: i64, dist: i64, charge: i64) -> bool {
    let moving = time - charge;
    moving * charge > dist
}

fn main() {
    let stdin = io::stdin();
    let mut scan = Scanner::new(stdin.lock());
    let _ = scan.next::<String>();
    let l1 = scan.next_split::<i64>();
    let _ = scan.next::<String>();
    let l2 = scan.next_split::<i64>();

    let mut options = 1;
    for (&time, &dist) in l1.iter().zip(l2.iter()) {
        options *= (0..=time)
            .map(|x| is_possible(time, dist, x))
            .filter(|x| *x)
            .count();
    }

    println!("{:?}", options);

    let p2time = conv(&l1);
    let p2dist = conv(&l2);

    let good = ternary_search_range_max(0, p2time, &|x| {
        let moving = p2time - x;
        moving * x
    });
    for s in good.0..=good.1 {
        if is_possible(p2time, p2dist, s) {
            let first = binary_search_range_min(0, good.1, &|x| is_possible(p2time, p2dist, x));
            let last = binary_search_range_max(good.1, p2time, &|x| is_possible(p2time, p2dist, x));
            println!("{}", last - first + 1);
            break;
        }
    }
}

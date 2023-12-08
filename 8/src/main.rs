use std::collections::*;
use std::io;
use std::io::BufRead;
use std::usize;

fn main() {
    let stdin = io::stdin();

    // let mut
    let mut inst = Vec::new();
    let mut order = HashMap::new();
    for (i, l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        if i == 0 {
            inst = l.chars().map(|x| x == 'R').collect::<Vec<_>>();
        } else if l.len() > 0 {
            let z = l
                .split(|x: char| return !x.is_alphanumeric())
                .filter(|x| x.len() > 0)
                .map(|x| x.to_owned())
                .collect::<Vec<_>>();
            order.insert(z[0].clone(), vec![z[1].clone(), z[2].clone()]);
        }
    }
    if order.contains_key("AAA") {
        let mut cur = "AAA".to_owned();
        let mut count = 0;

        while cur != "ZZZ" {
            let op = inst[count % inst.len()];
            count += 1;
            if op {
                cur = order[&cur][1].clone();
            } else {
                cur = order[&cur][0].clone();
            }
        }
        println!("P1: {:?}", count);
    }

    let nextz = |x: String, start: usize| {
        let mut cur = x.to_owned();
        let mut count = start;
        loop {
            let op = inst[count % inst.len()];

            count += 1;
            if op {
                cur = order[&cur][1].clone();
            } else {
                cur = order[&cur][0].clone();
            }
            if cur.chars().last().unwrap() == 'Z' {
                return (cur, count);
            }
        }
    };

    let find_cycles = |x: String, start: usize| {
        let mut count = start;

        let mut stops = vec![start];
        let mut seen = HashMap::new();

        let mut cur = x.to_owned();

        loop {
            if let Some(p) = seen.insert((cur.clone(), count % inst.len()), stops.len() - 1) {
                let cstart = stops[p];
                let mut l = stops.split_off(p);
                l.pop();
                let l = l.into_iter().collect::<VecDeque<_>>();
                return (l, count - cstart);
            }

            let n = nextz(cur.clone(), count);
            stops.push(n.1);

            cur = n.0.clone();
            count = n.1
        }
    };
    let mut p2 = 1;
    let mut starts = Vec::new();
    for x in order.keys() {
        if x.chars().last().unwrap() == 'A' {
            starts.push(nextz(x.clone(), 0));
        }
    }

    let mut cycles = starts
        .iter()
        .map(|x| find_cycles(x.0.clone(), x.1))
        .collect::<Vec<_>>();
    println!("{:?}", cycles);

    let mut idxs = starts.iter().map(|x| x.1).collect::<Vec<_>>();
    let mut prev_max = *idxs.iter().max().unwrap();

    loop {
        let l = starts.len();
        let mut updates = 0;
        for (i, x) in idxs.iter_mut().enumerate() {
            if *x < prev_max {
                updates += 1;
                let cl = cycles[i].1;
                let diff = prev_max - *x;
                if diff > (cl * 10) {
                    for x in cycles[i].0.iter_mut() {
                        *x += (diff / cl) * cl
                    }
                }

                let cv = &mut cycles[i].0;
                let d = cv.pop_front().unwrap();
                cv.push_back(d + cl);

                *x = d;

                prev_max = prev_max.max(*x);
            }
        }
        // println!("{:?}", idxs);
        if updates == 0 {
            break;
        }
    }
    println!("{}", idxs[0]);
}

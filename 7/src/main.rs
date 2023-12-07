use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn score_hand(hand: &str, p2: bool) -> (usize, usize) {
    let mut ans = HashMap::new();

    for (i, x) in hand.chars().enumerate() {
        *ans.entry(x).or_insert(0) += 1;
    }
    let mut s = hand.to_owned();
    s = s.replace('A', "F");
    s = s.replace('K', "E");
    s = s.replace('Q', "D");
    s = s.replace('T', "B");

    let mut handval = 0;
    if p2 {
        s = s.replace('J', "1");
        if let Some(mut p) = ans.remove(&'J') {
            if let Some(&max) = ans.values().max() {
                for x in ans.iter_mut() {
                    if *x.1 == max {
                        *x.1 += p;
                        p = 0;
                        break;
                    }
                }
            } else {
                ans.insert('J', p);
            }
        }
    } else {
        s = s.replace('J', "C");
    }

    for &p in ans.values() {
        if p > 1 {
            handval += 1 << (p * 2);
        }
    }

    (handval, usize::from_str_radix(&s, 16).unwrap())
}
fn main() {
    let stdin = io::stdin();
    let mut hands = Vec::new();
    for l in stdin.lock().lines() {
        let s = l.unwrap();
        let (hand, bid) = s.split_once(' ').unwrap();
        let bid = bid.parse::<usize>().unwrap();
        hands.push((hand.to_owned(), bid));
    }
    hands.sort_by_key(|x| score_hand(&x.0, false));
    let mut p1 = 0;
    for (i, v) in hands.iter().enumerate() {
        let i = i + 1;
        p1 += i * v.1;
    }
    println!("{:?}", p1);

    hands.sort_by_key(|x| score_hand(&x.0, true));
    let mut p2 = 0;
    for (i, v) in hands.iter().enumerate() {
        let i = i + 1;
        p2 += i * v.1;
    }
    println!("{:?}", p2);
}

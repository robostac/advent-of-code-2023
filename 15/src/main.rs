use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();

    for (_, l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        let mut p1 = 0;
        let mut boxes = vec![HashMap::new(); 256];

        for (i, p) in l.split(',').enumerate() {
            let mut hash = 0;
            let mut boxid = 0;
            let mut op = ' ';
            let mut focal = 0;
            let mut label = String::new();
            for c in p.chars() {
                if c == '-' || c == '=' {
                    boxid = hash;
                    op = c;
                } else if op == '=' {
                    focal = c.to_digit(10).unwrap() as usize;
                } else {
                    label.push(c);
                }
                hash = ((hash + (c as usize)) * 17) % 256;
            }
            p1 += hash;

            if op == '-' {
                boxes[boxid].remove(&label);
            } else {
                let mut this_idx = i;
                if let Some((idx, _)) = boxes[boxid].get(&label) {
                    this_idx = *idx;
                }

                boxes[boxid].insert(label, (this_idx, focal));
            }
        }
        println!("{:?}", p1);
        let mut p2 = 0;
        for i in 0..boxes.len() {
            if boxes[i].len() > 0 {
                let mut lenses = boxes[i].values().collect::<Vec<_>>();
                lenses.sort();
                for (j, v) in lenses.iter().enumerate() {
                    p2 += (i + 1) * (j + 1) * v.1;
                }
            }
        }
        println!("{}", p2);
    }
}

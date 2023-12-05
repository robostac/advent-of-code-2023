use scanner::Scanner;

use std::io;

fn map_range(mut start: i64, mut length: i64, map: &Vec<Vec<i64>>) -> Vec<(i64, i64)> {
    let mut result = Vec::new();
    let mut idx = 0;
    while length > 0 {
        if idx == map.len() {
            result.push((start, length));
            length = 0;
        } else {
            let src = map[idx][1];
            let dest = map[idx][0];
            let range = map[idx][2];
            let src_end = src + range;
            if start >= src_end {
                idx += 1;
            } else if src > start {
                let l = length.min(src - start + 1);
                result.push((start, l));
                start += l;
                length -= l;
            } else {
                let e = (start + length).min(src_end);
                let l = e - start;
                let offset = dest - src;
                result.push((start + offset, l));
                start += l;
                length -= l;
            }
        }
    }
    return result;
}

fn main() {
    let stdin = io::stdin();
    let mut scan = Scanner::new(stdin.lock());
    let mut seeds = Vec::new();
    let mut maps = Vec::new();
    let mut index = 0;
    let mut cmap = Vec::new();
    let mut cconv = Vec::new();
    while let Some(s) = scan.next::<String>() {
        if s == "map:" {
            index += 1;
            maps.push(cmap);
            cmap = Vec::new();
        }

        if let Some(p) = s.parse::<i64>().ok() {
            if index == 0 {
                seeds.push(p);
            } else {
                cconv.push(p);
                if cconv.len() == 3 {
                    cmap.push(cconv);
                    cconv = Vec::new();
                }
            }
        }
    }
    maps.push(cmap);
    for m in maps.iter_mut() {
        m.sort_by_key(|x| x[1]);
    }

    let mut p2 = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        p2.push((seeds[i], seeds[i + 1]));
    }

    for map in maps {
        for x in seeds.iter_mut() {
            *x = map_range(*x, 1, &map)[0].0;
        }
        let mut np2 = Vec::new();
        for x in p2 {
            np2.extend(map_range(x.0, x.1, &map));
        }
        p2 = np2;
    }

    println!("{:?}", seeds.iter().min().unwrap());
    println!("{:?}", p2.iter().min().unwrap().0);
}

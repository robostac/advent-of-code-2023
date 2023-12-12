use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn possible_spring(x: char) -> bool {
    return x == '?' || x == '#';
}

fn possible_gap(x: char) -> bool {
    return x == '?' || x == '.';
}

fn options(x: &[char], groups: &[usize], memo: &mut HashMap<(usize, usize), usize>) -> usize {
    let k = (x.len(), groups.len());
    if let Some(a) = memo.get(&k) {
        return *a;
    }
    if x.len() == 0 && groups.len() == 0 {
        return 1;
    }
    if x.len() == 0 {
        return 0;
    }
    let mut ans = 0;
    if possible_gap(x[0]) {
        ans += options(&x[1..], groups, memo);
    }
    if groups.len() > 0 && possible_spring(x[0]) {
        let glen = groups[0];
        if x.len() >= glen {
            if x[..glen].iter().all(|&c| possible_spring(c)) {
                if x.len() == glen {
                    ans += options(&x[(glen)..], &groups[1..], memo);
                } else if possible_gap(x[glen]) {
                    ans += options(&x[(glen + 1)..], &groups[1..], memo);
                }
            }
        }
    }
    memo.insert(k, ans);
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut p1 = 0;
    let mut p2 = 0;
    for (y, l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        let (springs, nums) = l.split_once(' ').unwrap();
        let nums = nums
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let springs = springs.chars().collect::<Vec<_>>();
        p1 += options(&springs, &nums, &mut HashMap::new());

        let mut p2nums = Vec::new();
        let mut p2springs = Vec::new();
        for _ in 0..5 {
            p2nums.extend(nums.iter().cloned());

            p2springs.extend(springs.iter().cloned());
            p2springs.push('?');
        }
        p2springs.pop();

        p2 += options(&p2springs, &p2nums, &mut HashMap::new());
    }
    println!("{:?}", p1);
    println!("{:?}", p2);
}

use std::collections::*;
use std::io;
use std::io::BufRead;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Rule {
    dest: String,
    condition: Option<(char, bool, i64)>,
}

impl Rule {
    fn new_parse(src: &str) -> Self {
        if let Some((a, b)) = src.split_once(':') {
            let c = a.chars().nth(0).unwrap();
            let op = a.chars().nth(1).unwrap();
            let val = a[2..].parse::<i64>().unwrap();
            Rule {
                dest: b.to_owned(),
                condition: Some((c, op == '>', val)),
            }
        } else {
            Rule {
                dest: src.to_owned(),
                condition: None,
            }
        }
    }

    fn apply(&self, input: &HashMap<char, i64>) -> Option<String> {
        if let Some((c, gt, val)) = self.condition {
            if let Some(&v) = input.get(&c) {
                if v != val && (v > val) == gt {
                    return Some(self.dest.clone());
                }
            }
            return None;
        } else {
            return Some(self.dest.clone());
        }
    }
}

fn main() {
    let stdin = io::stdin();

    let mut parts = Vec::new();
    let mut rules = HashMap::new();
    let mut parse1 = true;
    for (_y, mut l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        if l.len() == 0 {
            parse1 = false;
            continue;
        }

        l = l.replace('}', "");
        if parse1 {
            let (name, rules_str) = l.split_once('{').unwrap();

            let order = rules_str
                .split(',')
                .to_owned()
                .map(|x| Rule::new_parse(x))
                .collect::<Vec<_>>();
            rules.insert(name.to_owned(), order);
        } else {
            let part = l[1..]
                .split(',')
                .to_owned()
                .map(|x| {
                    let (a, b) = x.split_once('=').unwrap();
                    (a.chars().nth(0).unwrap(), b.parse::<i64>().unwrap())
                })
                .collect::<HashMap<_, _>>();
            parts.push(part);
        }
    }
    let mut accepted = Vec::new();
    for x in parts {
        let mut current = "in".to_owned();
        loop {
            match current.as_str() {
                "A" => {
                    accepted.push(x);
                    break;
                }
                "R" => {
                    break;
                }
                p => {
                    for z in rules[p].iter() {
                        if let Some(tgt) = z.apply(&x) {
                            current = tgt;
                            break;
                        }
                    }
                }
            }
        }
    }
    let mut p1 = 0;
    for p in accepted {
        p1 += p.values().sum::<i64>();
    }
    println!("{:?}", p1);

    let mut p2 = 0i64;

    let mut range = HashMap::new();
    for x in "xmas".chars() {
        range.insert(x, (1, 4000));
    }

    let mut queue = VecDeque::new();
    let mut accepted = Vec::new();
    queue.push_back(("in".to_owned(), range));
    while let Some((rule, mut range)) = queue.pop_front() {
        if rule == "A" {
            accepted.push(range);
            continue;
        } else if rule == "R" {
            continue;
        }
        let r = &rules[&rule];
        for p in r {
            if let Some((c, gt, val)) = p.condition {
                let mut new_range = range.clone();
                let e = new_range.entry(c).or_default();
                let oe = range.entry(c).or_default();
                if gt {
                    e.0 = e.0.max(val + 1);
                    oe.1 = oe.1.min(val);
                } else {
                    e.1 = e.1.min(val - 1);
                    oe.0 = oe.0.max(val);
                }
                if e.0 <= e.1 {
                    queue.push_back((p.dest.clone(), new_range));
                }
                if oe.0 > oe.1 {
                    break;
                }
            } else {
                queue.push_back((p.dest.clone(), range));
                break;
            }
        }
    }
    for x in accepted {
        let mut z = 1;
        for p in x.values() {
            z *= p.1 - p.0 + 1;
        }
        p2 += z;
    }
    println!("{}", p2);
}

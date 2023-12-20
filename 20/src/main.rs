use std::cell::RefCell;
use std::collections::*;
use std::io;
use std::io::BufRead;
use std::ops::*;
enum MType {
    Broadcast,
    Flip,
    Conj,
    Output,
}

struct ModuleImpl {
    targets: Vec<usize>,
    module: MType,
    state: usize,
    inputs: usize,
    debug: bool,
    cycles: HashMap<usize, usize>,
    prev: HashMap<usize, usize>,
}

type Module = std::cell::RefCell<ModuleImpl>;

struct Network {
    modules: Vec<Module>,
    broadcast: usize,
    id: HashMap<String, usize>,
}

impl ModuleImpl {
    fn new(mtype: MType) -> Self {
        ModuleImpl {
            targets: Vec::new(),
            module: mtype,
            state: 0,
            inputs: 0,
            debug: false,
            cycles: HashMap::new(),
            prev: HashMap::new(),
        }
    }

    fn add_child(&mut self, c: usize) {
        self.targets.push(c);
    }

    fn process_input(&mut self, input: bool, src: usize, bcount: usize) -> Option<bool> {
        if self.debug {
            if input {
                if let Some(p) = self.prev.insert(src, bcount) {
                    self.cycles.insert(src, bcount - p);
                    println!("dbG! {} {} {} {}", bcount, input, src, bcount - p);
                }
            }
        }
        let output;
        match self.module {
            MType::Broadcast => output = Some(input),

            MType::Flip => {
                if input == false {
                    self.state ^= 1;
                    output = Some(self.state == 1);
                } else {
                    output = None
                }
            }
            MType::Conj => {
                if input {
                    self.state |= 1 << src;
                } else {
                    self.state &= !(1 << src);
                }

                output = Some(self.state.count_ones() as usize != self.inputs);
            }
            MType::Output => {
                if input == false {
                    self.state += 1;
                }
                output = None
            }
        }
        output
    }
}

impl Network {
    fn new() -> Self {
        Network {
            broadcast: 0,
            modules: Vec::new(),
            id: HashMap::new(),
        }
    }

    fn add(&mut self, name: String, mtype: MType) {
        self.id.insert(name, self.modules.len());

        self.modules.push(RefCell::new(ModuleImpl::new(mtype)));
    }

    fn add_link(&mut self, n1: String, n2: String) {
        let id1 = self.id[&n1];
        let id2 = self.id[&n2];
        self.modules[id1].borrow_mut().add_child(id2);
        self.modules[id2].borrow_mut().inputs += 1;
    }

    fn push_button(&self, bcount: usize) -> (usize, usize) {
        let mut queue = VecDeque::new();
        queue.push_back((self.broadcast, false, 0));
        let mut low = 0;
        let mut high = 0;
        while let Some((tgt, signal, src)) = queue.pop_front() {
            // println!("{} {}", src, signal);
            if signal {
                high += 1;
            } else {
                low += 1;
            }
            let tgts = self.modules[tgt].borrow().targets.clone();
            if let Some(b) = self.modules[tgt]
                .borrow_mut()
                .process_input(signal, src, bcount)
            {
                for x in tgts.iter() {
                    queue.push_back((*x, b, tgt));
                }
            }
        }
        (low, high)
    }
}
fn lcm<
    T: Ord + Rem<Output = T> + PartialEq + Copy + TryFrom<i64> + Mul<Output = T> + Div<Output = T>,
>(
    first: T,
    second: T,
) -> T {
    first * second / gcd(first, second)
}

fn gcd<T: Ord + std::ops::Rem<Output = T> + PartialEq + Copy + TryFrom<i64>>(
    first: T,
    second: T,
) -> T {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0.try_into().ok().unwrap() {
            return min;
        }

        max = min;
        min = res;
    }
}

// fn lcm(fi

fn main() {
    let stdin = io::stdin();
    let mut links = Vec::new();
    let mut network = Network::new();

    for (_y, mut l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        let (name, targets) = l.split_once(" -> ").unwrap();
        let mut sidx = 1;
        let mt = match name.chars().nth(0).unwrap() {
            '%' => MType::Flip,
            '&' => MType::Conj,
            _ => {
                sidx = 0;
                MType::Broadcast
            }
        };
        network.add(name[sidx..].to_owned(), mt);
        for x in targets.split(", ") {
            links.push((name[sidx..].to_owned(), x.to_owned()));
        }
    }
    network.broadcast = network.id["broadcaster"];
    for (src, tgt) in links {
        if network.id.contains_key(&tgt) == false {
            network.add(tgt.clone(), MType::Output);
        }
        network.add_link(src, tgt);
    }
    let mut low = 0;
    let mut high = 0;
    let dbgid = network.id["kj"];
    network.modules[dbgid].borrow_mut().debug = true;
    for i in 0..1000 {
        let (l, h) = network.push_button(i);
        low += l;
        high += h;
        // println!();
    }

    println!("{}", low * high);
    let mut bcount = 1000;
    if let Some(&id) = network.id.get("rx") {
        while network.modules[dbgid].borrow().cycles.len() != network.modules[dbgid].borrow().inputs
        {
            network.push_button(bcount);
            bcount += 1;
        }
        let mut lc = 1;
        for p in network.modules[dbgid].borrow().cycles.iter() {
            lc = lcm(*p.1, lc);
        }
        println!("{:?}", lc);
    }
}

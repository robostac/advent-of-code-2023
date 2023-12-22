use std::collections::*;
use std::io;
use std::io::BufRead;

// fn lcm(fi

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Point { x, y, z }
    }
    fn add(&self, other: &Point) -> Self {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Brick {
    ends: [Point; 2],
    cells: Vec<Point>,
    bottom: i64,
    top: i64,
    id: usize,
    supports: HashSet<usize>,
    supported_by: HashSet<usize>,
}

impl Brick {
    fn new(mut inp: String, id: usize) -> Self {
        inp = inp.replace("~", ",");
        let points = inp
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mut b = Brick {
            ends: [
                Point::new(points[0], points[1], points[2]),
                Point::new(points[3], points[4], points[5]),
            ],
            cells: Vec::new(),
            bottom: i64::MAX,
            top: 0,
            id,
            supports: HashSet::new(),
            supported_by: HashSet::new(),
        };
        for x in b.ends[0].x.min(b.ends[1].x)..=b.ends[0].x.max(b.ends[1].x) {
            for y in b.ends[0].y.min(b.ends[1].y)..=b.ends[0].y.max(b.ends[1].y) {
                for z in b.ends[0].z.min(b.ends[1].z)..=b.ends[0].z.max(b.ends[1].z) {
                    b.cells.push(Point::new(x, y, z));
                }
            }
        }
        b.bottom = b.ends[0].z.min(b.ends[1].z);
        b.top = b.ends[0].z.max(b.ends[1].z);
        b
    }

    fn drop(&mut self, grid: &mut HashMap<Point, usize>) -> bool {
        if self.bottom == 1 {
            return false;
        }
        for p in self.cells.iter().filter(|p| p.z == self.bottom) {
            let down = Point::new(0, 0, -1).add(p);
            if grid.contains_key(&down) {
                return false;
            }
        }
        self.bottom -= 1;
        self.top -= 1;
        for p in self.cells.iter_mut() {
            grid.remove(&p);
            p.z -= 1;
            grid.insert(*p, self.id);
        }

        true
    }

    fn calc_supports(&mut self, grid: &HashMap<Point, usize>) {
        let mut ret = HashSet::new();
        for p in self.cells.iter().filter(|p| p.z == self.top) {
            let up = Point::new(0, 0, 1).add(p);
            if let Some(id) = grid.get(&up) {
                ret.insert(*id);
            }
        }

        self.supports = ret;
    }
}

fn calc_p2(idx: usize, b: &Vec<Brick>, order: &[usize]) -> HashSet<usize> {
    let mut ans = HashSet::new();
    ans.insert(idx);

    for p in order {
        if b[*p].supported_by.iter().len() > 0 && b[*p].supported_by.iter().all(|x| ans.contains(x))
        {
            ans.insert(*p);
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut bricks = Vec::new();

    for (_y, l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        bricks.push(Brick::new(l, _y));
    }
    let mut grid = HashMap::new();
    for x in bricks.iter() {
        for c in x.cells.iter() {
            grid.insert(*c, x.id);
        }
    }
    let mut in_motion = true;
    while in_motion {
        in_motion = false;
        for x in bricks.iter_mut() {
            in_motion |= x.drop(&mut grid);
        }
    }
    let mut supports = HashMap::new();
    for b in bricks.iter_mut() {
        b.calc_supports(&grid);
        for x in b.supports.iter() {
            supports.entry(*x).or_insert(HashSet::new()).insert(b.id);
        }
    }
    for x in supports {
        bricks[x.0].supported_by = x.1;
    }

    let mut count = 0;
    let mut order = (0..bricks.len()).collect::<Vec<_>>();
    order.sort_by_key(|x| bricks[*x].top);
    for b in bricks.iter() {
        let disintegrate = b.supports.iter().all(|x| bricks[*x].supported_by.len() > 1);
        if disintegrate {
            count += 1;
        }
    }
    println!("{:?}", count);
    let mut p2 = 0;
    for i in 0..bricks.len() {
        p2 += calc_p2(i, &bricks, &order).len() - 1;
    }
    println!("{}", p2);
}

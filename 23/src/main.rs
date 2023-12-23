use std::collections::*;
use std::io;
use std::io::BufRead;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    const fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
    fn add(&self, other: &Point) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

const LEFT: Point = Point::new(-1, 0);
const RIGHT: Point = Point::new(1, 0);
const UP: Point = Point::new(0, -1);
const DOWN: Point = Point::new(0, 1);

fn dfs(grid: &HashMap<Point, char>, seen: &mut HashSet<Point>, p: Point, end: Point) -> usize {
    // println!("{:?} {:?}", p, seen);
    if p == end {
        return seen.len();
    }
    let v = *grid.get(&p).unwrap();

    let mut options = Vec::new();

    match v {
        '>' => options.push(RIGHT),
        '<' => options.push(LEFT),
        '^' => options.push(UP),
        'v' => options.push(DOWN),
        _ => options.extend([LEFT, RIGHT, UP, DOWN]),
    };
    let mut ans = 0;
    for d in options {
        let np = d.add(&p);

        if grid.contains_key(&np) {
            if seen.insert(np) {
                ans = ans.max(dfs(grid, seen, np, end));
                seen.remove(&np);
            }
        }
    }
    ans
}

type Node = std::cell::RefCell<NodeImpl>;
#[derive(Debug, Clone)]
struct NodeImpl {
    _id: usize,
    children: Vec<(usize, usize)>,
    incoming: usize,
    value: i64,
    point: Point,
}

impl NodeImpl {
    fn new_filled(id: usize, value: i64) -> Node {
        std::cell::RefCell::new(NodeImpl {
            _id: id,
            children: Vec::new(),
            incoming: 0,
            value: value,
            point: Point::new(0, 0),
        })
    }

    fn add_edge(&mut self, other: usize, l: usize) {
        self.children.push((other, l));
    }
}
#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new(sz: usize, start: usize) -> Graph {
        Graph {
            nodes: (0..(sz + start))
                .map(|x| NodeImpl::new_filled(x, 0))
                .collect::<Vec<_>>(),
        }
    }

    fn add_directed_edge(&mut self, a: usize, b: usize, l: usize) {
        self.nodes[a].borrow_mut().add_edge(b, l);
        self.nodes[b].borrow_mut().incoming += 1;
    }

    fn add_undirected_edge(&mut self, a: usize, b: usize, l: usize) {
        self.add_directed_edge(a, b, l);
        self.add_directed_edge(b, a, l);
    }

    fn dfs(&self, n: usize, seen: &mut HashSet<usize>, distance: usize) -> usize {
        if self.nodes[n].borrow().value > 0 {
            return distance;
        }
        let mut ans = 0;
        for &(next, l) in self.nodes[n].borrow().children.iter() {
            if seen.insert(next) {
                ans = ans.max(self.dfs(next, seen, distance + l));
                seen.remove(&next);
            }
        }

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut mx = 0;
    let mut my = 0;
    let mut grid = HashMap::new();

    for (y, l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        for (x, c) in l.chars().enumerate() {
            mx = (x as i64).max(mx);
            if c == '#' {
                continue;
            }
            grid.insert(Point::new(x as i64, y as i64), c);
        }
        my = y as i64;
    }

    let mut graph = Graph::new(grid.len(), 1);
    let mut lookup = HashMap::new();
    for (i, v) in grid.keys().enumerate() {
        lookup.insert(*v, i + 1);
        graph.nodes[i + 1].borrow_mut().point = *v;
    }

    let start = *grid
        .iter()
        .filter(|x| x.0.y == 0 && *x.1 == '.')
        .next()
        .unwrap()
        .0;
    let end = *grid
        .iter()
        .filter(|x| x.0.y == my && *x.1 == '.')
        .next()
        .unwrap()
        .0;

    println!("{}", dfs(&grid, &mut HashSet::new(), start, end));

    let mut queue = VecDeque::new();
    queue.push_back((start, LEFT));
    queue.push_back((start, RIGHT));
    queue.push_back((start, DOWN));
    queue.push_back((start, UP));
    let mut seen_edges = HashSet::new();
    let mut seen_nodes = HashSet::new();
    seen_nodes.insert(start);
    graph.nodes[lookup[&end]].borrow_mut().value = 1;
    while let Some((p, d)) = queue.pop_front() {
        let mut prev = p;
        let mut np = p.add(&d);
        let mut l = 1;
        while grid.contains_key(&np) {
            let neigbours = [LEFT, RIGHT, UP, DOWN]
                .iter()
                .filter_map(|x| {
                    let neigh = np.add(x);
                    if grid.contains_key(&neigh) && neigh != prev {
                        Some(neigh)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if neigbours.len() > 1 || np == end {
                let mut p1 = lookup[&p];
                let mut p2 = lookup[&np];
                if p1 > p2 {
                    std::mem::swap(&mut p1, &mut p2);
                }
                if seen_edges.insert((p1, p2)) {
                    graph.add_undirected_edge(p1, p2, l);
                }
                if seen_nodes.insert(np) {
                    queue.push_back((np, LEFT));
                    queue.push_back((np, RIGHT));
                    queue.push_back((np, DOWN));
                    queue.push_back((np, UP));
                }
                break;
            } else if neigbours.len() == 1 {
                prev = np;
                np = neigbours[0];
                l += 1;
            } else {
                break;
            }
        }
    }
    println!("{:?}", graph.dfs(lookup[&start], &mut HashSet::new(), 0));
}

use std::collections::*;
use std::io;
use std::io::BufRead;
use std::process::id;

// fn lcm(fi

/* Returns true if there is a path from source 's' to sink 't' in
residual graph. Also fills parent[] to store the path */

fn bfs(links: &Vec<Vec<i64>>, s: usize, t: usize, parent: &mut Vec<usize>) -> bool {
    let mut visited = HashSet::new();

    // Create a queue, enqueue source vertex and mark source vertex
    // as visited
    let mut queue = VecDeque::new();
    queue.push_back(s);

    visited.insert(s);
    parent[s] = 0;
    while let Some(u) = queue.pop_front() {
        for (p, &v) in links[u].iter().enumerate() {
            if v > 0 && visited.insert(p) {
                queue.push_back(p);
                parent[p] = u;
            }
        }
    }
    return visited.contains(&t);
}

fn dfs(links: &Vec<Vec<i64>>, s: usize, visited: &mut Vec<bool>) {
    visited[s] = true;
    for (p, &v) in links[s].iter().enumerate() {
        if visited[p] == false && v > 0 {
            dfs(links, p, visited);
        }
    }
    // void dfs(int rGraph[V][V],
}
// // Prints the minimum s-t cut
fn minCut(
    links: &HashMap<usize, HashSet<usize>>,
    s: usize,
    t: usize,
    n: usize,
) -> (usize, usize, usize) {
    let mut rGraph = vec![vec![0i64; n + 1]; n + 1];
    for u in links.iter() {
        for v in u.1.iter() {
            rGraph[*u.0][*v] = 1;
        }
    }
    let mut parent = vec![0; n + 1];
    let mut u = 0;
    let mut cuts = 0;
    while bfs(&rGraph, s, t, &mut parent) {
        // 	// Augment the flow while there is a path from source to sink

        // 		// Find minimum residual capacity of the edges along the
        // 		// path filled by BFS. Or we can say find the maximum flow
        // 		// through the path found.
        let mut path_flow = i64::MAX;
        let mut v = t;
        while v != s {
            u = parent[v];
            path_flow = path_flow.min(rGraph[u][v]);
            v = parent[v];
        }
        cuts += 1;
        // 		// update residual capacities of the edges and reverse edges
        // 		// along the path
        v = t;
        while v != s {
            u = parent[v];
            rGraph[u][v] -= path_flow;
            rGraph[v][u] += path_flow;
            v = parent[v];
        }
    }

    // 	// Flow is maximum now, find vertices reachable from s
    let mut visited = vec![false; n + 1];
    dfs(&rGraph, s, &mut visited);
    let visit = visited.iter().filter(|x| **x).count();

    (visit, n - visit, cuts)
    // 	bool visited[V];
    // 	memset(visited, false, sizeof(visited));
    // 	dfs(rGraph, s, visited);

    // 	// Print all edges that are from a reachable vertex to
    // 	// non-reachable vertex in the original graph
    // 	for (int i = 0; i < V; i++)
    // 	for (int j = 0; j < V; j++)
    // 		if (visited[i] && !visited[j] && graph[i][j])
    // 			cout << i << " - " << j << endl;

    // 	return;
    // }
}

fn main() {
    let stdin = io::stdin();

    let mut wires: HashMap<String, usize> = HashMap::new();
    let mut links = HashMap::new();

    let mut idx = 1;
    for (_y, l) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        let l = l.replace(':', "");
        let t = l.split_whitespace().collect::<Vec<_>>();
        for p in t.iter() {
            if wires.contains_key(&(*p).to_owned()) == false {
                wires.insert((*p).to_owned(), idx);
                idx += 1;
            }
        }
        let idx = wires[t[0]];
        for i in t.iter().skip(1) {
            let idx2 = wires[*i];
            links.entry(idx).or_insert(HashSet::new()).insert(idx2);
            links.entry(idx2).or_insert(HashSet::new()).insert(idx);
        }
    }
    for i in 2..idx {
        let ans = minCut(&links, 1, i, wires.len());
        if ans.2 == 3 {
            println!("{:?} {}", ans, ans.0 * ans.1);
            break;
        }
    }
}

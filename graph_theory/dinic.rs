use std::{cmp::min, collections::VecDeque, io};

fn read_input() -> Vec<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    let input: Vec<&str> = input.split(" ").collect();
    input
        .into_iter()
        .map(|x| {
            x.parse::<usize>()
                .expect(&format!("Could not parse: {:?}", x))
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    node: usize, // To what node does it point?
    flow: i32,
    capacity: i32,
    rev: usize,
}

#[derive(Debug, Clone)]
struct Graph {
    size: usize,
    adjlist: Vec<Vec<Edge>>,
    level: Vec<i32>,
}

impl Graph {
    fn from(size: usize) -> Graph {
        let adjlist = vec![vec![]; size];
        let level = vec![];
        Graph {
            size,
            adjlist,
            level,
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        let a = Edge {
            node: to,
            flow: 0,
            capacity: 1,
            rev: self.adjlist[to].len(),
        };
        let b = Edge {
            node: from,
            flow: 0,
            capacity: 0,
            rev: self.adjlist[from].len(),
        };
        self.adjlist[from].push(a);
        self.adjlist[to].push(b);
    }

    /// Can source reach sink
    /// Sets levels for each node while searching
    fn bfs(&mut self, source: usize, sink: usize) -> bool {
        self.level = vec![-1; self.size];

        let mut queue = VecDeque::new();

        self.level[source] = 0;
        queue.push_back(source);

        while let Some(parent) = queue.pop_front() {
            for child_index in 0..self.adjlist[parent].len() {
                let child = self.adjlist[parent][child_index];

                if self.level[child.node] < 0 && child.flow < child.capacity {
                    self.level[child.node] = self.level[parent] + 1;
                    queue.push_back(child.node);
                }
            }
        }

        if self.level[sink] < 0 {
            false
        } else {
            true
        }
    }

    fn send_flow(
        &mut self,
        node: usize,
        flow: i32,
        sink: usize,
        edges_visited: &mut Vec<usize>,
    ) -> Option<i32> {
        if node == sink {
            return Some(flow);
        }

        for _ in edges_visited[node]..self.adjlist[node].len() {
            let mut edge = self.adjlist[node][edges_visited[node]];

            if self.level[edge.node] == self.level[node] + 1 && edge.flow < edge.capacity {
                let curr_flow = min(flow, edge.capacity - edge.flow);
                let next_flow = self.send_flow(edge.node, curr_flow, sink, edges_visited);

                if let Some(flow) = next_flow {
                    edge.flow += flow;

                    self.adjlist[node][edges_visited[node]].flow += flow;
                    self.adjlist[edge.node][edge.rev].flow -= flow;

                    return Some(flow);
                }
            }

            edges_visited[node] += 1;
        }

        return None;
    }

    fn dinic_max_flow(&mut self, source: usize, sink: usize) -> i32 {
        let mut total = 0;

        // while available paths
        while self.bfs(source, sink) {
            let mut edges_visited = vec![0; self.size];

            // println!("{:?}", self.adjlist);

            while let Some(i) = self.send_flow(source, 1, sink, &mut edges_visited) {
                total += i;
            }
        }

        total
    }
}

/// Execute a test case
fn test() {
    let input = read_input();
    let amount_nodes = input[0];
    let amount_edges = input[1];

    let mut rgraph = Graph::from(amount_nodes);

    for _ in 0..amount_edges {
        let position = read_input();
        rgraph.add_edge(position[0] - 1, position[1] - 1);
    }

    let data = read_input();
    let source = data[0] - 1;
    let sink = data[1] - 1;
    let unique_paths_limit = data[2] as i32;

    let unique_paths = rgraph.dinic_max_flow(source, sink);
    println!("{:?}", unique_paths);

    if unique_paths > unique_paths_limit {
        println!("YES");
    } else {
        println!("NO");
    }
}

/// Solve for New Friends
/// https://www.hackerearth.com/practice/algorithms/graphs/maximum-flow/practice-problems/algorithm/new-friends/
/// https://www.geeksforgeeks.org/dinics-algorithm-maximum-flow/
fn main() {
    for _ in 0..read_input()[0] {
        test()
    }
}

use std::cmp::Ordering;
use std::{collections::BinaryHeap, io};

fn read_input() -> Vec<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Reading input from STDIN
                                                // println!("{:?}", input);
    input = input.trim().to_string();
    let input: Vec<&str> = input.split(" ").collect();
    input
        .into_iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Edge {
    node: usize,
    cost: usize,
}
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Graph = Vec<Vec<Edge>>;

fn build_graph(amount_nodes: usize, amount_edges: usize) -> Graph {
    let mut alist = vec![vec![]; amount_nodes];

    for _ in 0..amount_edges {
        let position = read_input();
        alist[position[0] - 1].push(Edge {
            node: position[1] - 1,
            cost: position[2],
        });
        alist[position[1] - 1].push(Edge {
            node: position[0] - 1,
            cost: position[2],
        });
    }

    alist
}

// Prim greedy algorithm for finding Minimal Spanning Tree
fn prim(graph: Graph) -> usize {
    let mut minimal_cost = 0;
    let mut visited_nodes = vec![false; graph.len()];
    let mut heap: BinaryHeap<Edge> = BinaryHeap::new();

    // Start with first node without cost
    heap.push(Edge { node: 0, cost: 0 });

    while let Some(p) = heap.pop() {
        // skip if already visited node
        if visited_nodes[p.node] {
            continue;
        }
        visited_nodes[p.node] = true;
        minimal_cost += p.cost;

        // Iterate over adjacended
        for c in graph[p.node].iter() {
            // if not already visited
            if visited_nodes[c.node] == false {
                heap.push(*c);
            }
        }
    }

    minimal_cost
}

// minimum spanning tree
fn main() {
    let input = read_input();
    let amount_nodes = input[0];
    let amount_edges = input[1];

    let graph = build_graph(amount_nodes, amount_edges);
    println!("{}", prim(graph))
}

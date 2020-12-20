use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;

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

/// Build directed graph
fn build_graph(amount_nodes: usize, amount_edges: usize) -> Graph {
  let mut alist = vec![vec![]; amount_nodes];

  for _ in 0..amount_edges {
    let position = read_input();
    alist[position[0] - 1].push(Edge {
      node: position[1] - 1,
      cost: position[2],
    });
  }

  alist
}

// Dijkstra's Algorithm
fn dijkstra(graph: Graph) -> Vec<usize> {
  let mut heap = BinaryHeap::new();
  let mut distances = vec![usize::MAX; graph.len()];
  let mut visited = vec![false; graph.len()];

  heap.push(Edge { node: 0, cost: 0 });
  distances[0] = 0;

  while let Some(parent) = heap.pop() {
    if visited[parent.node] {
      continue;
    }
    visited[parent.node] = true;

    for child in graph[parent.node].iter() {
      let next = Edge {
        cost: parent.cost + child.cost,
        node: child.node,
      };
      if next.cost < distances[child.node] {
        distances[child.node] = next.cost;
        heap.push(next);
      }
    }
  }
  distances.remove(0);
  distances
}

fn main() {
  let input = read_input();
  let amount_nodes = input[0];
  let amount_edges = input[1];

  let graph = build_graph(amount_nodes, amount_edges);
  let distances = dijkstra(graph);
  println!(
    "{}",
    distances
      .iter()
      .map(|d| d.to_string())
      .collect::<Vec<String>>()
      .join(" ")
  );
}

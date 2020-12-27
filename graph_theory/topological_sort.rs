use std::{
  cmp::Reverse,
  collections::{BinaryHeap, VecDeque},
  io,
};

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

// Build adjacency list from input
fn build_adjlist(amount_nodes: usize, amount_edges: usize) -> Vec<Vec<usize>> {
  let mut alist = vec![vec![]; amount_nodes];

  for _ in 0..amount_edges {
    let pos = read_input();
    alist[pos[0] - 1].push(pos[1] - 1);
  }

  alist
}

/// Topical sort using Breadth First Search.
/// If there are multiple ordering return the lexicographically smallest one
fn topilogical_sort(adjlist: &Vec<Vec<usize>>) -> Vec<usize> {
  let mut sorted_nodes = vec![];
  let mut in_degree = vec![0; adjlist.len()];
  let mut visited_nodes = vec![false; adjlist.len()];
  let mut heap = BinaryHeap::new();

  for node in adjlist.iter() {
    for child in node.iter() {
      in_degree[*child] += 1;
    }
  }

  for i in 0..in_degree.len() {
    if in_degree[i] == 0 {
      visited_nodes[i] = true;
      heap.push(Reverse(i));
    }
  }

  while let Some(node) = heap.pop() {
    sorted_nodes.push(node.0);

    for child in adjlist[node.0].iter() {
      if !visited_nodes[*child] {
        in_degree[*child] -= 1;
        if in_degree[*child] == 0 {
          visited_nodes[*child] = true;
          heap.push(Reverse(*child))
        }
      }
    }
  }

  sorted_nodes
}

fn main() {
  let counts = read_input();
  let amount_nodes = counts[0];
  let amount_edges = counts[1];
  let adjlist = build_adjlist(amount_nodes, amount_edges);

  let sorted_nodes = topilogical_sort(&adjlist);

  let result = sorted_nodes
    .iter()
    .map(|x| (x + 1).to_string())
    .collect::<Vec<String>>()
    .join(" ");

  println!("{}", result);
}

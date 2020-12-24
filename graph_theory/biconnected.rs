use std::collections::HashSet;
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

// Build undirected adjacency list from input
fn build_adjlist(amount_nodes: usize, amount_edges: usize) -> Vec<Vec<usize>> {
  let mut alist = vec![vec![]; amount_nodes];

  for _ in 0..amount_edges {
    let pos = read_input();
    alist[pos[1]].push(pos[0]);
    alist[pos[0]].push(pos[1]);
  }

  alist
}

/// Get a list of bi components calculated using articulation points
fn dfs(
  node: usize,
  disc: &mut Vec<usize>,
  low: &mut Vec<usize>,
  adjlist: &Vec<Vec<usize>>,
  parent: &mut Vec<Option<usize>>,
  aps: &mut Vec<bool>,
  visited_nodes: &mut Vec<bool>,
  time: &mut usize,
  edges: &mut Vec<[usize; 2]>,
  bicomp: &mut Vec<HashSet<usize>>,
) {
  disc[node] = *time;
  low[node] = *time;
  *time += 1;

  visited_nodes[node] = true;
  for child in adjlist[node].iter() {
    if visited_nodes[*child] == false {
      parent[*child] = Some(node);
      edges.push([node, *child]);
      dfs(
        *child,
        disc,
        low,
        adjlist,
        parent,
        aps,
        visited_nodes,
        time,
        edges,
        bicomp,
      );

      // if child has been discovered earlier than set parent to lowest child discovery time
      if low[node] > low[*child] {
        low[node] = low[*child];
      }

      // If root with multiple children it is an AP
      if parent[node].is_none() && adjlist[node].len() <= 1 {
        continue;
      }
      // If grand parent does not exists and child earliest discovery is later than discovery of parent
      if parent[node].is_some() && low[*child] < disc[node] {
        continue;
      }

      // is ap
      let mut comp = HashSet::new();
      while let Some(edge) = edges.pop() {
        for node in edge.iter() {
          comp.insert(*node);
        }
        if edge[0] == node && edge[1] == *child {
          break;
        }
      }
      bicomp.push(comp);
      aps[node] = true;
    } else if low[node] > disc[*child] {
      if let Some(parent_node) = parent[node] {
        if parent_node == *child {
          continue;
        }
      }
      low[node] = disc[*child];
      edges.push([node, *child]);
    }
  }
}

/// Wrapper around dfs for finding biconnected components and makes sure it also gets unconnected components
fn biconnected_components(adjlist: Vec<Vec<usize>>) -> Vec<HashSet<usize>> {
  let mut visited_nodes = vec![false; adjlist.len()];
  let mut disc = vec![usize::MAX; adjlist.len()];
  let mut low = vec![usize::MAX; adjlist.len()];
  let mut parents = vec![None; adjlist.len()];
  let mut aps = vec![false; adjlist.len()];
  let mut bicomp = vec![];
  let mut edges = vec![];
  for i in 0..visited_nodes.len() {
    if visited_nodes[i] == false {
      dfs(
        i,
        &mut disc,
        &mut low,
        &adjlist,
        &mut parents,
        &mut aps,
        &mut visited_nodes,
        &mut 1,
        &mut edges,
        &mut bicomp,
      );
      if edges.len() > 0 {
        let mut comp = HashSet::new();
        for edge in edges.iter() {
          for node in edge {
            comp.insert(*node);
          }
        }
        bicomp.push(comp);
      }
    }
  }

  bicomp
}

/// Print out amount vertices even and odd biconnected components
fn main() {
  let counts = read_input();
  let amount_nodes = counts[0];
  let amount_edges = counts[1];
  let adjlist = build_adjlist(amount_nodes, amount_edges);

  let bi_comps = biconnected_components(adjlist);

  let mut odd_count = 0;
  let mut even_count = 0;

  for comp in bi_comps.iter() {
    if comp.len() % 2 == 0 {
      even_count += 1
    } else {
      odd_count += 1
    }
  }
  println!("{} {}", odd_count, even_count)
}

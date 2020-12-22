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

// Build adjacency list from input
fn build_adjlist(amount_nodes: usize, amount_edges: usize) -> Vec<Vec<usize>> {
  let mut alist = vec![vec![]; amount_nodes];

  for _ in 0..amount_edges {
    let pos = read_input();
    alist[pos[1]].push(pos[0]);
    alist[pos[0]].push(pos[1]);
  }

  alist
}

/// Get articulation points and bridges using dfs
fn dfs(
  node: usize,
  disc: &mut Vec<usize>,
  low: &mut Vec<usize>,
  adjlist: &Vec<Vec<usize>>,
  parent: &mut Vec<Option<usize>>,
  aps: &mut Vec<bool>,
  bridges: &mut Vec<[usize; 2]>,
  visited_nodes: &mut Vec<bool>,
  time: &mut usize,
) {
  disc[node] = *time;
  low[node] = *time;
  *time += 1;

  visited_nodes[node] = true;
  for child in adjlist[node].iter() {
    if visited_nodes[*child] == false {
      parent[*child] = Some(node);
      dfs(
        *child,
        disc,
        low,
        adjlist,
        parent,
        aps,
        bridges,
        visited_nodes,
        time,
      );

      // if parent has been discovered earlier than child set
      if low[node] > low[*child] {
        low[node] = low[*child];
      }
      // low[node] = cmp::min(low[node], low[child])

      // If child has been discovered earlier than parent it is an bridge
      if low[*child] > disc[node] {
        bridges.push([node, *child])
      }
      // If root with multiple children it is an AP
      if parent[node] == None && adjlist[node].len() > 1 {
        aps[node] = true;
      }
      // If grand parent exists and child earliest discovery is earlier than discovery of parent is is an AP
      if parent[node].is_some() && low[*child] >= disc[node] {
        aps[node] = true;
      }
    } else if low[node] > disc[*child] {
      if let Some(parent_node) = parent[node] {
        if parent_node == *child {
          // println!("P {} {}", node, child);
          continue;
        }
      }
      low[node] = disc[*child];
    }
  }
}

fn main() {
  let counts = read_input();
  let amount_nodes = counts[0];
  let amount_edges = counts[1];
  let adjlist = build_adjlist(amount_nodes, amount_edges);

  let mut disc = vec![0; adjlist.len()];
  let mut low = vec![usize::MAX; adjlist.len()];
  let mut visited_nodes = vec![false; adjlist.len()];
  let mut aps = vec![false; adjlist.len()];
  let mut bridges = vec![];
  dfs(
    0,
    &mut disc,
    &mut low,
    &adjlist,
    &mut vec![None; adjlist.len()],
    &mut aps,
    &mut bridges,
    &mut visited_nodes,
    &mut 1,
  );
  let mut aps: Vec<String> = aps
    .iter()
    .enumerate()
    .filter(|x| *x.1)
    .map(|x| x.0.to_string())
    .collect();
  aps.sort();

  println!("{}", aps.len());
  println!("{}", aps.join(" "));

  println!("{}", bridges.len());
  bridges.sort();
  for bridge in bridges.iter() {
    let res = bridge
      .iter()
      .map(|i| i.to_string())
      .collect::<Vec<String>>()
      .join(" ");
    println!("{}", res);
  }
}

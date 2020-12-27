use std::{collections::HashMap, io};

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
    alist[pos[0] - 1].push(pos[1] - 1);
  }

  alist
}

fn reverse_adjlist(adjlist: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
  let mut reversed_adjlist = vec![vec![]; adjlist.len()];

  for node in 0..adjlist.len() {
    for child in adjlist[node].iter() {
      reversed_adjlist[*child].push(node);
    }
  }

  reversed_adjlist
}

/// Build stack with finish times
fn dfs(
  node: usize,
  adjlist: &Vec<Vec<usize>>,
  visited_nodes: &mut Vec<bool>,
  stack: &mut Vec<usize>,
) {
  visited_nodes[node] = true;

  for child in adjlist[node].iter() {
    if !visited_nodes[*child] {
      dfs(*child, adjlist, visited_nodes, stack)
    }
  }
  stack.push(node);
}

fn assign(
  node: usize,
  root: usize,
  adjlist: &Vec<Vec<usize>>,
  components: &mut HashMap<usize, Vec<usize>>,
  assigned: &mut Vec<bool>,
) {
  components.get_mut(&root).unwrap().push(node);
  assigned[node] = true;
  for child in adjlist[node].iter() {
    if !assigned[*child] {
      assign(*child, root, adjlist, components, assigned)
    }
  }
}

/// Kosaraju's Linear time algorithm to find Strongly Connected Components
fn kosaraju(adjlist: Vec<Vec<usize>>) -> HashMap<usize, Vec<usize>> {
  let reversed_adjlist = reverse_adjlist(&adjlist);
  let mut components = HashMap::new();
  let mut visited_nodes = vec![false; adjlist.len()];
  let mut stack = vec![];

  for root in 0..adjlist.len() {
    if !visited_nodes[root] {
      dfs(root, &adjlist, &mut visited_nodes, &mut stack);
    }
  }
  let mut assigned = vec![false; adjlist.len()];

  while let Some(source) = stack.pop() {
    if !assigned[source] {
      components.insert(source, vec![]);
      assign(
        source,
        source,
        &reversed_adjlist,
        &mut components,
        &mut assigned,
      );
    }
  }

  components
}

/// Print out amount vertices even and odd minus each other
fn main() {
  let counts = read_input();
  let amount_nodes = counts[0];
  let amount_edges = counts[1];
  let adjlist = build_adjlist(amount_nodes, amount_edges);

  let strong_components = kosaraju(adjlist);

  let mut odd_count = 0;
  let mut even_count = 0;

  for comp in strong_components.iter() {
    if comp.1.len() % 2 == 0 {
      even_count += comp.1.len() as i32;
    } else {
      odd_count += comp.1.len() as i32;
    }
  }
  println!("{}", odd_count - even_count)
}

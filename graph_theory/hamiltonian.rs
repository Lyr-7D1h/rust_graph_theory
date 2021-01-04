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

/// Build an undirected adjacency matrix
fn build_amatrix(amount_nodes: usize, amount_edges: usize) -> Vec<Vec<bool>> {
  let mut amatrix = vec![vec![false; amount_nodes]; amount_nodes];

  for _ in 0..amount_edges {
    let position = read_input();
    amatrix[position[0]][position[1]] = true;
    amatrix[position[1]][position[0]] = true;
  }

  amatrix
}

/// Mutate permutation to the next greatest lexicographic order and returns boolean if succeeded or not
/// https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
fn next_permutation(permutation: &mut Vec<usize>) -> bool {
  let mut k = 0;
  let mut k_set = false;
  for i in 0..permutation.len() - 1 {
    if permutation[i] < permutation[i + 1] {
      k_set = true;
      k = i;
    }
  }

  if !k_set {
    return false;
  }

  let mut l = 0;
  for i in 0..permutation.len() {
    if permutation[k] < permutation[i] {
      l = i;
    }
  }

  permutation.swap(k, l);
  permutation[k + 1..].reverse();
  true
}

/// Check if permutation works for amatrix
fn is_valid_permutation(permutation: &Vec<usize>, amatrix: &Vec<Vec<bool>>) -> bool {
  for i in 0..permutation.len() - 1 {
    if amatrix[permutation[i]][permutation[i + 1]] == false {
      return false;
    }
  }
  return true;
}

/// Check if hamiltonian path is possible for given adjacency matrix
/// It checks every possible permutation of nodes and returns true if one works otherwise false.
fn is_hamiltonian_brute(amatrix: &Vec<Vec<bool>>) -> bool {
  let mut permutation = vec![];

  for i in 0..amatrix.len() {
    permutation.push(i)
  }

  if is_valid_permutation(&permutation, amatrix) {
    return true;
  }

  while next_permutation(&mut permutation) == true {
    if is_valid_permutation(&permutation, amatrix) {
      return true;
    }
  }

  return false;
}

/// Check if hamiltonian path is possible
/// using Dynamic Programming
fn is_hamiltonian_dp(amatrix: &Vec<Vec<bool>>) -> bool {
  let mut dp = vec![vec![false; 1 << amatrix.len()]; amatrix.len()];
  for i in 0..amatrix.len() {
    dp[i][1 << i] = true;
  }

  for i in  0..1<<amatrix.len() {
    for ii in 0..amatrix.len() {
      if i & (1 << ii) > 0 {
        for iii in 0..amatrix.len() {
          if i & (1 << iii) > 0 && amatrix[iii][ii] && iii != ii && dp[iii][i^(1<<ii)] {
            dp[ii][i] = true;
              break
          }
        }
      } 
    }
  }

  for i in 0..amatrix.len() {
    if dp[i][(1<<amatrix.len())-1] {
      return true
    }
  }

  return false;
}

fn in_stack_dfs(node: usize, in_stack: &mut Vec<bool>, in_stack_amount: usize, amatrix: &Vec<Vec<bool>>) -> bool {
  if in_stack_amount == amatrix.len() {
    return true
  }
  for i in 0..amatrix.len() {
    if amatrix[node][i] && in_stack[i] == false {
      in_stack[i] = true;
      if in_stack_dfs(i, in_stack, in_stack_amount + 1, amatrix) {
        return true
      }
      in_stack[i] = false;
    }
  }
  return false
}

/// Check if hamiltonian path is possible
/// using Depth First Search
fn is_hamiltonian_dfs(amatrix: &Vec<Vec<bool>>) -> bool {
    let mut in_stack = vec![false; amatrix.len()];
  for i in 0..amatrix.len() {
      in_stack[i] = true;
    if in_stack_dfs(i, &mut in_stack, 1, amatrix) {
      return true 
    }
    in_stack[i] = false;
  }
return false 
}

fn main() {
  let input = read_input();
  let amount_nodes = input[0];
  let amount_edges = input[1];
  let amatrix = build_amatrix(amount_nodes, amount_edges);

  // let is_hamil = is_hamiltonian_brute(&amatrix);
  // let is_hamil = is_hamiltonian_dp(&amatrix);
  let is_hamil = is_hamiltonian_dfs(&amatrix);

  if is_hamil {
    println!("YES");
  } else {
    println!("NO");
  }
}

#[cfg(test)]
mod tests {
  use super::next_permutation;

  #[test]
  fn next_permutation_works() {
    let mut permutation = vec![1, 2, 3, 4];
    next_permutation(&mut permutation);
    assert_eq!(permutation, vec![1, 2, 4, 3]);
    next_permutation(&mut permutation);
    assert_eq!(permutation, vec![1, 3, 2, 4]);

    let mut count = 0;
    while next_permutation(&mut permutation) {
      count += 1;
    }
    assert_eq!(count, 24 - 3); // 24 total permutations (4!) already tested 3
    assert_eq!(permutation, vec![4, 3, 2, 1])
  }
}

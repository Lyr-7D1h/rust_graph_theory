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

type Matrix = Vec<Vec<usize>>;
type VMatrix = Vec<Vec<bool>>;

/// Build undirected graph (adjacency list)
fn build_matrix(amount_rows: usize) -> Matrix {
  let mut matrix = vec![];

  for _ in 0..amount_rows {
    matrix.push(read_input())
  }

  matrix
}

/// Depth first search for seeing if there is a path in matrix
fn dfs_can_reach(
  x: usize,
  y: usize,
  amount_rows: usize,
  amount_columns: usize,
  matrix: &Matrix,
  vmatrix: &mut VMatrix,
  dest_x: usize,
  dest_y: usize,
) -> bool {
  if x == dest_x && y == dest_y {
    return true;
  }

  if x >= amount_columns || y >= amount_rows {
    return false;
  }
  if vmatrix[y][x] {
    return false;
  }
  if matrix[y][x] == 0 {
    return false;
  }

  vmatrix[y][x] = true;

  if dfs_can_reach(
    x + 1,
    y,
    amount_rows,
    amount_columns,
    matrix,
    vmatrix,
    dest_x,
    dest_y,
  ) {
    return true;
  }
  if dfs_can_reach(
    x,
    y + 1,
    amount_rows,
    amount_columns,
    matrix,
    vmatrix,
    dest_x,
    dest_y,
  ) {
    return true;
  }
  if x == 0 {
    return false;
  }
  if dfs_can_reach(
    x - 1,
    y,
    amount_rows,
    amount_columns,
    matrix,
    vmatrix,
    dest_x,
    dest_y,
  ) {
    return true;
  }
  if y == 0 {
    return false;
  }
  if dfs_can_reach(
    x,
    y - 1,
    amount_rows,
    amount_columns,
    matrix,
    vmatrix,
    dest_x,
    dest_y,
  ) {
    return true;
  }
  return false;
}

fn main() {
  let input = read_input();
  let amount_rows = input[0];
  let amount_columns = input[1];
  let matrix = build_matrix(amount_rows);

  let mut visited_matrix = vec![vec![false; amount_columns]; amount_rows];
  if dfs_can_reach(
    0,
    0,
    amount_rows,
    amount_columns,
    &matrix,
    &mut visited_matrix,
    amount_rows - 1,
    amount_columns - 1,
  ) {
    println!("Yes");
  } else {
    println!("No");
  }
}

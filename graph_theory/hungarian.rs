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

fn build_matrix(size: usize) -> Vec<Vec<usize>> {
    let mut matrix = vec![vec![0; size]; size];
    for column in 0..size {
        let row_values = read_input();
        for row in 0..size {
            matrix[row][column] = row_values[row];
        }
    }

    matrix
}

/// NOTE: not fully working - not finishing due to time constraints
/// Using Adjacancy Matrix Solve from: https://brilliant.org/wiki/hungarian-matching/
/// And https://brc2.com/the-algorithm-workshop/
fn hungarian(matrix: Vec<Vec<usize>>, size: usize) -> usize {
    let mut matrix_clone = matrix.clone();

    // STEP 1
    // Reduce each row with the minimal value of that row
    for row in matrix_clone.iter_mut() {
        let min = row.iter().min().unwrap();
        *row = row.iter().map(|x| x - min).collect();
    }

    // Saved zeros
    let mut stars = vec![vec![false; size]; size];
    let mut primes = vec![vec![false; size]; size];

    // Setup cover each row and column should only contain one '0'
    let mut row_cover = vec![0; size];
    let mut column_cover = vec![0; size];

    // STEP 2
    // Find a zero
    // Skip if there already is a zero defined in that row or column
    for row in 0..size {
        for column in 0..size {
            if matrix_clone[row][column] == 0 && row_cover[row] == 0 && column_cover[column] == 0 {
                stars[row][column] = true;
                row_cover[row] = 1;
                column_cover[column] = 1;
            }
        }
    }

    // Reset cover
    row_cover = vec![0; size];
    column_cover = vec![0; size];

    let mut verify = true;

    loop {
        if verify {
            // STEP 3
            // Check how many columns have zeros if this is equal to the size then finish
            for row in 0..size {
                for column in 0..size {
                    if stars[row][column] {
                        column_cover[column] = 1;
                    }
                }

                let mut count = 0;
                for value in column_cover.iter() {
                    if *value == 1 {
                        count += 1;
                    }
                }

                if count == size {
                    let mut res = 0;
                    for row in 0..size {
                        for column in 0..size {
                            if stars[row][column] {
                                res += matrix[row][column];
                            }
                        }
                    }

                    return res;
                }
            }
        }

        // STEP 4
        // Find an uncovered zero and prime it
        let mut uncovered = None;

        'outer: for row in 0..size {
            for column in 0..size {
                if stars[row][column] == false && row_cover[row] == 0 && column_cover[column] == 0 {
                    uncovered = Some((row, column));
                    primes[row][column] = true;
                    break 'outer;
                }
            }
        }

        if let None = uncovered {
            // STEP 6
            // find minimum uncovered value
            let mut min = usize::MAX;
            for row in 0..size {
                if row_cover[row] == 1 {
                    continue;
                }
                for column in 0..size {
                    if column_cover[column] == 1 {
                        continue;
                    }
                    let value = matrix_clone[row][column];
                    if value < min {
                        min = value;
                    }
                }
            }

            for row in 0..size {
                for column in 0..size {
                    if row_cover[row] == 1 {
                        matrix_clone[row][column] += min
                    }
                    if column_cover[column] == 0 {
                        matrix_clone[row][column] -= min
                    }
                }
            }

            // next iteration and skip step 3
            verify = false;
            continue;
        }

        let (i, j) = uncovered.unwrap();

        // If there's a starred zero in the same row
        // - Cover row of uncovered zero from [Step 4]
        // - Uncover column of starred zero
        // - Repeat [Step 4]
        if let Some(j) = (0..size).find(|&j| stars[i][j]) {
            row_cover[i] = 1;
            column_cover[j] = 0;
            verify = false;
            continue;
        }

        // Step 5
        // construct an alternating path of stars and primes
        let mut path = vec![(i, j)];
        loop {
            let (_, j) = path[path.len() - 1];

            let next_star = (0..size).find(|&i| stars[i][j]);

            if let None = next_star {
                break;
            }

            let i = next_star.unwrap();
            path.push((i, j));

            let j = (0..size).find(|&j| primes[i][j]).unwrap();
            path.push((i, j));
        }

        for (i, j) in path {
            stars[i][j] = primes[i][j]
        }

        row_cover = vec![0; size];
        column_cover = vec![0; size];

        primes = vec![vec![false; size]; size];
        verify = true;
    }
}

fn main() {
    let size = read_input()[0];

    let matrix = build_matrix(size);

    println!("{}", hungarian(matrix, size));
}

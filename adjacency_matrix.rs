use std::io;

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

fn main() {
    let input = read_input();
    let nnodes = input[0];
    let nedges = input[1];

    let mut amatrix: Vec<Vec<usize>> = vec![vec![]; nnodes];

    for _ in 0..nedges {
        let position = read_input();
        let x = position[0] - 1;
        let y = position[1] - 1;
        amatrix[x][y] = 1;
    }

    // println!("{:?}", amatrix);
    // println!("{}, {}", nnodes, nedges);

    let nqueries = read_input()[0];
    for _ in 0..nqueries {
        let position = read_input();
        let x = position[0] - 1;
        let y = position[1] - 1;
        if amatrix[x][y] == 1 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

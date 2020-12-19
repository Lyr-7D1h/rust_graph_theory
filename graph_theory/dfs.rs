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

// Build Adjacency list
fn get_alist(amount_nodes: usize, amount_edges: usize) -> Vec<Vec<usize>> {
    let mut alist = vec![vec![]; amount_nodes];

    for _ in 0..amount_edges {
        let position = read_input();
        alist[position[0] - 1].push(position[1] - 1);
        alist[position[1] - 1].push(position[0] - 1);
    }

    alist
}

fn main() {
    let input = read_input();
    let amount_nodes = input[0];
    let amount_edges = input[1];
    let alist = get_alist(amount_nodes, amount_edges);
    let mut visited_nodes = vec![false; amount_nodes];

    let root_node = read_input()[0] - 1;

    let mut stack: Vec<usize> = vec![];
    stack.push(root_node);
    visited_nodes[root_node] = true;

    while stack.len() != 0 {
        let parent = match stack.pop() {
            Some(parent) => parent,
            None => break,
        };
        for child in alist[parent].iter() {
            let child = *child;
            if visited_nodes[child] == false {
                stack.push(child);
                visited_nodes[child] = true
            }
        }
    }

    // How many nodes are unvisited?
    let mut count = 0;
    for has_visited in visited_nodes.iter() {
        if *has_visited == false {
            count += 1;
        }
    }
    println!("{}", count);
}

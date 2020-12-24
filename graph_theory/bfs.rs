use std::{collections::VecDeque, io};

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
fn get_alist(amount_nodes: usize) -> Vec<Vec<usize>> {
    let mut alist = vec![vec![]; amount_nodes];

    for _ in 0..amount_nodes - 1 {
        let position = read_input();
        alist[position[0] - 1].push(position[1] - 1);
        alist[position[1] - 1].push(position[0] - 1);
    }

    alist
}

fn main() {
    let amount_nodes = read_input()[0];
    let alist = get_alist(amount_nodes);
    let mut levels = vec![0; amount_nodes];
    let mut visited_nodes = vec![false; amount_nodes];

    let root_node = 0;

    let mut queue: VecDeque<usize> = VecDeque::with_capacity(amount_nodes - 1);
    queue.push_back(root_node);
    levels[root_node] = 1;
    visited_nodes[root_node] = true;

    while queue.len() != 0 {
        let parent = match queue.pop_front() {
            Some(parent) => parent,
            None => break,
        };
        for child in alist[parent].iter() {
            let child = *child;
            if visited_nodes[child] == false {
                queue.push_back(child);
                levels[child] = levels[parent] + 1;
                // println!("{:?}", child);
                visited_nodes[child] = true
            }
        }
    }

    // How many nodes are on search_level?
    let search_level = read_input()[0];
    let mut count = 0;
    for level in levels.iter() {
        if *level == search_level {
            count += 1;
        }
    }
    println!("{}", count);

    // println!("{:?}", alist);
    // println!("{:?}", levels);
}

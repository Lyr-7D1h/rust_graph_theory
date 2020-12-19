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

#[derive(Clone)]
struct Edge {
    node: usize,
    cost: usize,
}

type Graph = Vec<Vec<Edge>>;

fn build_graph(amount_nodes: usize, amount_edges: usize) -> Graph {
    let mut alist = vec![vec![]; amount_nodes];

    for _ in 0..amount_edges {
        let position = read_input();
        alist[position[0] - 1].push(Edge {
            node: position[1] - 1,
            cost: position[2],
        });
        alist[position[1] - 1].push(Edge {
            node: position[0] - 1,
            cost: position[2],
        });
    }

    alist
}

// Floyed Warshall Algorithm
fn wfi(graph: Graph) -> Vec<i32> {
    unimplemented!()
}
// minimum spanning tree
fn main() {
    let input = read_input();
    let amount_nodes = input[0];
    let amount_edges = input[1];

    let graph = build_graph(amount_nodes, amount_edges);
    let distances = wfi(graph);
    println!(
        "{}",
        distances
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

use std::{collections::HashMap, io};

fn read_input() -> (Vec<usize>, Vec<char>) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    let input: Vec<&str> = input.split(" ").collect();

    let mut chars = vec![];
    let mut values = vec![];

    for x in input.iter() {
        match x.parse::<usize>() {
            Ok(x) => values.push(x),
            Err(_) => chars.push(x.parse::<char>().unwrap()),
        }
    }

    (values, chars)
}

/// Build undirected adjacency map
fn build_amatrix(amount_edges: usize, s: &char, t: &char) -> (Vec<Vec<usize>>, (usize, usize)) {
    let mut amatrix = vec![vec![0; amount_edges]; amount_edges];
    let mut map = HashMap::new();

    for _ in 0..amount_edges {
        let pos = read_input();
        let x = pos.1[0];
        let y = pos.1[1];
        let cost = pos.0[0];

        let l = map.len();
        let x = *map.entry(x).or_insert(l);
        let l = map.len();
        let y = *map.entry(y).or_insert(l);

        amatrix[x][y] = cost;
        amatrix[y][x] = cost;
    }

    (
        amatrix[..map.len()]
            .iter()
            .map(|x| x[..map.len()].to_owned())
            .collect(),
        (*map.get(s).unwrap(), *map.get(t).unwrap()),
    )
}

fn contract(amatrix: &Vec<Vec<usize>>, bin: &Vec<bool>, s: &mut usize, t: &mut usize) -> usize {
    let mut dist = vec![0; amatrix.len()];
    let mut visited = vec![false; amatrix.len()];
    let mut mincut = 0;

    for _ in 0..amatrix.len() {
        let mut k = None;
        let mut maxc: Option<usize> = None;
        for j in 0..amatrix.len() {
            if let Some(m) = maxc {
                if dist[j] <= m {
                    continue;
                }
            }
            if !bin[j] && !visited[j] {
                k = Some(j);
                maxc = Some(dist[j]);
            }
        }
        if let None = k {
            return mincut;
        }
        *s = *t;
        *t = k.unwrap();
        mincut = maxc.unwrap();
        visited[k.unwrap()] = true;
        for j in 0..amatrix.len() {
            if !bin[j] && !visited[j] {
                dist[j] += amatrix[k.unwrap()][j];
            }
        }
    }

    return mincut;
}

/// Stoer wagner algorithm
/// NOTE: not fully working but moving on due to time constraints
/// https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm
fn stoer_wagner(mut amatrix: Vec<Vec<usize>>, mut s: usize, mut t: usize) -> usize {
    let mut mincut = usize::MAX;
    // let mut edge = vec![vec![0]; amatrix.len()];
    let mut bin = vec![false; amatrix.len()];

    for _ in 0..amatrix.len() {
        let ans = contract(&amatrix, &bin, &mut s, &mut t);
        bin[t] = true;
        if ans == 0 {
            break;
        }
        if mincut > ans {
            mincut = ans
        }
        for j in 0..amatrix.len() {
            if !bin[j] {
                amatrix[j][s] += amatrix[j][t];
                amatrix[s][j] = amatrix[j][s];
            }
        }
    }

    mincut
}

fn main() {
    let input = read_input();
    let amount_edges = input.0[0];
    let s = input.1[0];
    let t = input.1[1];

    let (amatrix, (s, t)) = build_amatrix(amount_edges, &s, &t);

    println!("{}", stoer_wagner(amatrix, s, t))
}

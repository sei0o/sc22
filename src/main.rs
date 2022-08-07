use petgraph::{
    algo::floyd_warshall,
    dot::{Config, Dot},
    graph::NodeIndex,
    Graph,
};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
};

fn main() {
    let edges = edges();
    println!("edges: {}", edges.len());

    let mut g: Graph<u32, ()> = Graph::new();
    let mut nodes: HashMap<u32, NodeIndex<u32>> = HashMap::new();
    for (f, t) in &edges {
        let fnode = match nodes.get(f) {
            Some(&n) => n,
            None => {
                let n = g.add_node(*f);
                nodes.insert(*f, n);
                n
            }
        };

        let tnode = match nodes.get(t) {
            Some(&n) => n,
            None => {
                let n = g.add_node(*t);
                nodes.insert(*t, n);
                n
            }
        };

        g.add_edge(fnode, tnode, ());
    }
    println!("generated graph");

    let distances = floyd_warshall(&g, |_| 1).unwrap();
    let n_edge = distances.len() as f64;
    let total_distance = distances.iter().fold(0.0, |acc, (_, d)| match d {
        &std::i32::MAX => acc,
        _ => acc + *d as f64,
    });
    let max_distance = distances
        .iter()
        .map(|(_, d)| match d {
            &std::i32::MAX => 0,
            _ => *d,
        })
        .max()
        .unwrap();
    let avg = total_distance / n_edge;
    println!("average distance between two nodes: {}", avg);
    println!(
        "diameter = max distance between two nodes: {}",
        max_distance
    );

    // export to graphviz
    let dot = format!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
    let dst = std::env::temp_dir().join("sc22.dot");
    let mut file = File::create(dst).unwrap();
    file.write_all(dot.as_bytes()).unwrap();
}

fn edges() -> Vec<(u32, u32)> {
    let csv = File::open("dataset/edges.csv").unwrap();
    let reader = BufReader::new(csv);
    reader
        .lines()
        .filter_map(|x| match x {
            Ok(x) => {
                let mut sp = x.split(',');
                let f = sp.next().unwrap().parse().unwrap();
                let t = sp.next().unwrap().parse().unwrap();

                // filter out accounts with larger than 1000
                if f > 1000 || t > 1000 {
                    return None;
                }

                Some((f, t))
            }
            Err(_) => None,
        })
        .collect()
}

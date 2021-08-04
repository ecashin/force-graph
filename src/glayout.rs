use ndarray::{Array1, Array2, ArrayView1, Axis};
use rand::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn initial_positions(n: usize, n_dims: usize) -> Array2<f32> {
    let mut a = Array2::zeros((n, n_dims));
    let mut rng = thread_rng();

    for i in 0..n {
        for j in 0..n_dims {
            a[[i, j]] = rng.gen_range(0.0..=1.0);
        }
    }
    a
}

#[derive(Debug)]
pub struct Edge {
    pub src: usize,
    pub dst: usize,
}

pub fn add_edges(n_vertices: usize, max_degree: usize) -> Vec<Edge> {
    let mut rng = thread_rng();
    let mut degrees = vec![0; n_vertices];
    let mut edges: HashSet<(usize, usize)> = HashSet::new();
    for src in 0..n_vertices {
        let degree = rng.gen_range(1..max_degree);
        let mut n_attempts = 3;
        while degrees[src] < degree && n_attempts > 0 {
            let dst = rng.gen_range(0..n_vertices);
            if src == dst {
                continue;
            }
            if dst < src {
                if degrees[dst] >= max_degree {
                    continue;
                }
                edges.insert((dst, src));
            } else {
                edges.insert((src, dst));
            }
            degrees[src] += 1;
            degrees[dst] += 1;
            n_attempts -= 1;
        }
    }
    edges
        .into_iter()
        .map(|(src, dst)| Edge { src, dst })
        .collect()
}

pub fn print_js(pos: Array2<f32>, edges: Vec<Edge>) {
    println!("(function () {{");
    println!("    var g = G.graph()");
    println!("    var nodes = []");
    println!("    var edge, node");
    for row in pos.rows() {
        println!("    node = G.node({}, {{\"color\": \"black\"}})", row);
        println!("    nodes.push(node); node.addTo(g)");
    }
    for Edge { src, dst } in edges.into_iter() {
        println!(
            "    edge = G.edge([nodes[{}], nodes[{}]], {{color: \"blue\"}})",
            src, dst
        );
        println!("    edge.addTo(g)");
    }
    println!("    g.renderIn(\"graph\")");
    println!("}})()\n");
}

fn dist(x1: &ArrayView1<f32>, x2: &ArrayView1<f32>) -> f32 {
    let diff = x1 - x2;

    (&diff * &diff).sum().sqrt()
}

pub fn force_graph(pos: &mut Array2<f32>, edges: &[Edge], n_iters: usize) {
    let ideal_dist: f32 = 1.0 / pos.nrows() as f32;
    let mut edges_by_node: HashMap<usize, Vec<usize>> = HashMap::new();
    const CTR_WEIGHT: f32 = 0.1;
    let spread_weight: f32 = 0.0 / pos.len_of(Axis(0)) as f32;
    let repel_weight: f32 = 0.0 / pos.len_of(Axis(0)) as f32;

    for Edge { src, dst } in edges {
        let mut v = edges_by_node.entry(*src).or_insert_with(Vec::new);
        v.push(*dst);
        v = edges_by_node.entry(*dst).or_insert_with(Vec::new);
        v.push(*src);
    }

    for iter_no in 0..n_iters {
        let mut new_positions = pos.to_owned();
        for (i, row) in pos.outer_iter().enumerate() {
            println!("{} row:{} orig_row:{:?}", iter_no, i, row);
            let mut repel: Array1<f32> = Array1::zeros(row.len());
            for other in pos.rows() {
                if row == other {
                    continue;
                }
                repel = repel + repel_weight * (&row - &other) * dist(&row, &other);
            }
            println!("{} row:{} repel:{:?}", iter_no, i, repel);
            let ctr = (&row * &row).sum().sqrt() * (-1.0 * &row) * CTR_WEIGHT;
            println!("{} row:{} ctr:{:?}", iter_no, i, ctr);
            let mut spread: Array1<f32> = Array1::zeros(row.len());
            for (point, neighbors) in &edges_by_node {
                let point = &pos.index_axis(Axis(0), *point);
                for i in neighbors {
                    let neighbor = &pos.index_axis(Axis(0), *i);
                    let obs_dist = dist(&point, &neighbor);
                    let direction = (neighbor - point) / obs_dist;
                    spread = spread + spread_weight * (obs_dist - ideal_dist) * direction;
                }
            }
            println!("{} row:{} spread:{:?}", iter_no, i, spread);
            let mut new_row = new_positions.index_axis_mut(Axis(0), i);
            new_row += &(repel + ctr + spread);
            println!("{} row:{} new_row:{:?}", iter_no, i, new_row);
        }
        pos.assign(&new_positions);
    }
}

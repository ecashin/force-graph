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
    pub src: u32,
    pub dst: u32,
}

pub fn add_edges(n_vertices: usize, max_degree: usize) -> Vec<Edge> {
    let mut rng = thread_rng();
    let mut degrees = vec![0; n_vertices];
    let mut edges: HashSet<(u32, u32)> = HashSet::new();
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
                edges.insert((dst as u32, src as u32));
            } else {
                edges.insert((src as u32, dst as u32));
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

fn dist(x1: &ArrayView1<f32>, x2: &ArrayView1<f32>) -> f32 {
    let diff = x1 - x2;

    (&diff * &diff).sum().sqrt()
}

// https://cs.brown.edu/people/rtamassi/gdhandbook/chapters/force-directed.pdf
pub fn force_graph(pos: &mut Array2<f32>, edges: &[Edge], n_iters: usize, ctr_weight: f32) {
    let ideal_dist: f32 = 1.0; // c_2 in rtamassi
    let mut edges_by_node: HashMap<u32, Vec<u32>> = HashMap::new();
    let spread_weight: f32 = 1.0; // C_1 == 2 in rtamassi
    let repel_weight: f32 = 0.01; // C_3 in rtamassi
    const SIG_DISTANCE: f32 = 0.00001;

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
                let distance = dist(&row, &other);
                repel = repel + (repel_weight / distance.powi(2)) * (&row - &other);
            }
            let distance = (&row * &row).sum().sqrt();
            let ctr = if distance > SIG_DISTANCE {
                let direction = &row / distance;
                -ctr_weight * &direction
            } else {
                &row * 0.0
            };
            let mut spread: Array1<f32> = Array1::zeros(row.len());
            for (point, neighbors) in &edges_by_node {
                let point = &pos.index_axis(Axis(0), *point as usize);
                for j in neighbors {
                    if *j == i as u32 {
                        continue;
                    }
                    let neighbor = &pos.index_axis(Axis(0), *j as usize);
                    let obs_dist = dist(&point, &neighbor);
                    let direction = (neighbor - point) / obs_dist;
                    let n_others = neighbors.len() as f32;
                    spread = spread
                        + (spread_weight / n_others)
                            * (obs_dist / ideal_dist).log(10.0)
                            * direction;
                }
            }
            let mut new_row = new_positions.index_axis_mut(Axis(0), i);
            new_row += &(repel + ctr + spread);
        }
        pos.assign(&new_positions);
    }
}

#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        caas: [[usize; m + 1]; n]
    };
    let mut costs = Vec::new();
    for row_size in 1..=n {
        for row_indexes in (0..n).combinations(row_size) {
            let mut cost = 0;
            let mut sums = vec![0usize; m];
            // let rows = row_indexes.clone().iter().map(|x| x.to_string()).join(" ");
            // dbg!(rows);
            for row_index in row_indexes {
                let row = &caas[row_index];
                cost += row[0];
                for (i, aij) in row[1..].iter().enumerate() {
                    sums[i] += aij;
                }
            }
            // let ss = sums.clone().iter().map(|x| x.to_string()).join(" ");
            // dbg!(&cost, ss);
            if sums.iter().all(|&sum| sum >= x) {
                costs.push(cost);
            }
        }
    }
    // dbg!(&costs);
    println!(
        "{}",
        if costs.is_empty() {
            "-1".to_string()
        } else {
            costs.iter().min().unwrap().to_string()
        }
    )
}

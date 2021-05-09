#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    };
    let mut hm: HashMap<usize, Vec<usize>> = HashMap::new();
    for size in 1..=n {
        for arr in aa.iter().enumerate().combinations(size) {
            let sum = arr.iter().map(|(_, &a)| a).sum::<usize>() % 200;
            let ii = arr.iter().map(|(i, _)| i + 1).collect_vec();
            if let Some(jj) = hm.get(&sum) {
                print!(
                    "Yes\n{} {}\n{} {}",
                    jj.len(),
                    jj.iter().map(|j| j.to_string()).join(" "),
                    ii.len(),
                    ii.iter().map(|i| i.to_string()).join(" ")
                );
                return;
            } else {
                hm.insert(sum, ii);
            }
        }
    }
    println!("No");
}

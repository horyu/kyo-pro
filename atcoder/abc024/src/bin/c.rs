#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        _n: usize,
        d: usize,
        k: usize,
        llrr: [(Isize1, Isize1); d],
        sstt: [(Isize1, Isize1); k]
    };
    let mut righters = vec![];
    let mut lefters = vec![];
    let mut postions = vec![0; k];
    let mut results = vec![0; k];
    for i in 0..k {
        let (s, t) = sstt[i];
        if s < t {
            righters.push(i);
        } else {
            lefters.push(i);
        }
        postions[i] = s;
    }
    for (day, &(l, r)) in llrr.iter().enumerate() {
        let day = day + 1;
        for i in (0..lefters.len()).rev() {
            let pos_index = lefters[i];
            let pos = &mut postions[pos_index];
            if (l < *pos) && (*pos <= r) {
                *pos = l;
                if *pos <= sstt[pos_index].1 {
                    results[pos_index] = day;
                    lefters.remove(i);
                }
            }
        }
        for i in (0..righters.len()).rev() {
            let pos_index = righters[i];
            let pos = &mut postions[pos_index];
            if (l <= *pos) && (*pos < r) {
                *pos = r;
                if *pos >= sstt[pos_index].1 {
                    results[pos_index] = day;
                    righters.remove(i);
                }
            }
        }
        if lefters.is_empty() && righters.is_empty() {
            break;
        }
    }
    for result in results {
        println!("{}", result);
    }
}

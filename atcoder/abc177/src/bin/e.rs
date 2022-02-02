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
    let pf = PrimeFact::new(1e6 as usize);
    let mut all_hm = HashMap::new();
    for hm in  aa.into_iter().map(|a| pf.get(a)) {
        for &k in hm.keys() {
            *all_hm.entry(k).or_insert(0) += 1;
        }
    }
    if all_hm.values().all(|&cnt| cnt == 1) {
        println!("pairwise coprime");
    } else if all_hm.values().all(|&cnt| cnt != n) {
        println!("setwise coprime");
    } else {
        println!("not coprime");
    }
}

// https://algo-logic.info/prime-fact/
struct PrimeFact {
    spf: Vec<usize>,
}
impl PrimeFact {
    fn new(n: usize) -> Self {
        let mut spf = (0..=n).collect_vec();
        for i in (2..=n).take_while(|x| x * x <= n) {
            if spf[i] == i {
                for j in (i..=n).step_by(i) {
                    if spf[j] == j {
                        spf[j] = i;
                    }
                }
            }
        }
        Self { spf }
    }

    fn get(&self, mut n: usize) -> HashMap<usize, usize> {
        let mut hm = HashMap::new();
        while n != 1 {
            *hm.entry(self.spf[n]).or_insert(0) += 1usize;
            n /= self.spf[n];
        }
        hm
    }
}

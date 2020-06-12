#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        _n: usize,
        a: Usize1,
        b: Usize1,
        k: usize,
        pp: [Usize1 ;k]
    };
    let mut set = std::collections::HashSet::new();
    set.insert(&a);
    set.insert(&b);
    for p in &pp {
        if set.contains(p) {
            println!("NO");
            return;
        }
        set.insert(p);
    }
    println!("YES");
}

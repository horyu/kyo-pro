#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
    };
    let mut sqrt = 1usize;
    while (sqrt + 1).pow(2) <= n {
        sqrt += 1;
    }
    let mut viewed = std::collections::HashSet::new();
    let mut sub = 0;
    for a in 2..=sqrt {
        if viewed.contains(&a) {
            continue;
        }
        let mut num = a.pow(2);
        while num <= n {
            viewed.insert(num);
            sub += 1;
            num *= a;
        }
    }
    println!("{}", n - sub);
}

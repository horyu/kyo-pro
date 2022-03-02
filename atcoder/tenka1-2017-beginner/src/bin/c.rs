#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: isize
    };
    // 4xyz = N(xy + yz + zx)
    // z(4xy - Nx - Ny)) = Nxy
    // z = Nxy/(4xy - N(x+y))
    for x in 1..=3500 {
        for y in x..=3500 {
            let denominator = 4 * x * y - n * (x + y);
            if denominator > 0 && n * x * y % denominator == 0 {
                let z = n * x * y / denominator;
                println!("{x} {y} {z}");
                return;
            }
        }
    }
}

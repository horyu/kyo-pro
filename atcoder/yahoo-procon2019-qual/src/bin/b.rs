#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        aabb: [(Usize1, Usize1); 3]
    };
    let mut arr = [0; 4];
    for (a, b) in aabb {
        arr[a] += 1;
        arr[b] += 1;
    }
    let tf = arr.iter().all(|&x| x <= 2);
    println!("{}", ["NO", "YES"][tf as usize]);
}

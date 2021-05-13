#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        c: usize,
        mut nnmmll: [[usize; 3]; c]
    };
    let mut arr = [0usize; 3];
    for mut nml in nnmmll {
        nml.sort_unstable();
        for i in 0usize..3 {
            arr[i] = arr[i].max(nml[i]);
        }
    }
    println!("{}", arr.iter().product::<usize>());
}

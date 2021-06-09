#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
    };
    let sum = (1..=n).sum::<usize>();
    for i in 2..n {
        if sum % i == 0 {
            println!("BOWWOW");
            return;
        }
    }
    if sum == 1 {
        println!("BOWWOW");
    } else {
        println!("WANWAN");
    }
}

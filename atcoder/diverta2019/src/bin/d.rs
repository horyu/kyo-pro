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
    let mut rs = 0;
    for i in 1usize..=n.sqrt() {
        let nn = n - i;
        let m = nn / i;
        if m == 0 {
            break;
        }
        if n / m == n % m {
            rs += m;
        }
    }
    println!("{rs}");
}

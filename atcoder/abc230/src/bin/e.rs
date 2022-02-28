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
    // https://oeis.org/search?q=1%2C+3%2C+5%2C+8%2C+10%2C+14%2C+16%2C+20%2C+23%2C+27&language=english&go=Search
    // a(n) = 2*(Sum_{i=1..floor(sqrt(n))} floor(n/i)) - floor(sqrt(n))^2
    let mut rs = 0;
    let sqrt = n.sqrt();
    for i in 1..=sqrt {
        rs += 2 * (n / i);
    }
    rs -= sqrt * sqrt;
    println!("{rs}");
}

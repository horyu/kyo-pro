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
    // // https://oeis.org/search?q=1%2C5%2C11%2C23%2C33%2C57%2C71%2C103%2C130&sort=&language=english&go=Search
    // // Sum_{i=1..floor(sqrt(n))} i*floor(n/i)*(1+floor(n/i))
    // //  - [floor(sqrt(n))*(1+floor(sqrt(n)))/2]^2;
    // let mut rs = 0usize;
    // let n_sqrt = n.sqrt();
    // for i in 1..=n_sqrt {
    //     rs += i * (n / i) * (1 + n / i);
    // }
    // rs -= ((n_sqrt * (1 + n_sqrt)) / 2).pow(2u32);

    // https://kanpurin.hatenablog.com/entry/2021/02/26/110437
    let mut rs = 0;
    for i in 1..=n {
        rs += i * (1 + n / i) * (n / i) / 2;
    }
    println!("{rs}");
}

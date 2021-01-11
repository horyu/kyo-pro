#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: u32,
        aa: [usize; 2usize.pow(n)]
    };
    let nn = 2usize.pow(n - 1);
    let mae = aa[..nn].iter().position_max().unwrap();
    let ato = nn + aa[nn..].iter().position_max().unwrap();
    let i = if aa[mae] < aa[ato] { mae } else { ato };
    println!("{}", i + 1);
}

#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n],
        bb: [Usize1; k],
    };
    let bb: HashSet<_> = bb.into_iter().collect();
    let mut kouho = vec![];
    let mut max = 0;
    for i in 0..n {
        let a = aa[i];
        #[allow(clippy::comparison_chain)]
        if max < a {
            max = a;
            kouho = vec![i];
        } else if max == a {
            kouho.push(i);
        }
    }
    // dbg!(&kouho, &bb);
    let tf = kouho.into_iter().any(|i| bb.contains(&i));
    println!("{}", ["No", "Yes"][tf as usize]);
}

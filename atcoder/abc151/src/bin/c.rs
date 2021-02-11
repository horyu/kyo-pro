#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        psps: [(Usize1, String); m],
    };
    let mut acs = vec![false; n];
    let mut was = vec![0; n];
    for (p, s) in psps {
        if acs[p] {
            continue;
        }
        if s == "AC" {
            acs[p] = true;
        } else {
            was[p] += 1;
        }
    }
    let (ac, wa) = acs
        .into_iter()
        .zip(was.into_iter())
        .fold(
            (0, 0),
            |(ac, wa), (tf, num)| if tf { (ac + 1, wa + num) } else { (ac, wa) },
        );
    println!("{} {}", ac, wa);
}

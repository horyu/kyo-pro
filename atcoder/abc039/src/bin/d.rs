#![allow(unused_imports)]
use itertools::iproduct;
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        w: usize,
        sss: [Chars; h]
    };
    let hh = h - 1;
    let ww = w - 1;
    let mut aaa = vec![vec![false; w]; h];
    for (i, aa) in aaa.iter_mut().enumerate() {
        for (j, a) in aa.iter_mut().enumerate() {
            if mawari(i, j, hh, ww).all(|(y, x)| sss[y][x] == '#') {
                *a = true;
            }
        }
    }
    let mut bbb = vec![vec!['.'; w]; h];
    for (i, aa) in aaa.iter().enumerate() {
        for (j, a) in aa.iter().enumerate() {
            if *a {
                for (y, x) in mawari(i, j, hh, ww) {
                    bbb[y][x] = '#';
                }
            }
        }
    }
    if sss == bbb {
        println!("possible");
        for aa in aaa {
            println!(
                "{}",
                aa.into_iter()
                    .map(|a| ['.', '#'][a as usize])
                    .collect::<String>()
            )
        }
    } else {
        println!("impossible");
    }
}

fn mawari(
    i: usize,
    j: usize,
    hh: usize,
    ww: usize,
) -> itertools::Product<std::vec::IntoIter<usize>, std::vec::IntoIter<usize>> {
    let tate = match i {
        0 if hh == 0 => vec![0],
        0 => vec![0, 1],
        i if i == hh => vec![i - 1, i],
        i => vec![i - 1, i, i + 1],
    };
    let yoko = match j {
        0 if ww == 0 => vec![0],
        0 => vec![0, 1],
        j if j == ww => vec![j - 1, j],
        j => vec![j - 1, j, j + 1],
    };
    tate.into_iter().cartesian_product(yoko.into_iter())
}

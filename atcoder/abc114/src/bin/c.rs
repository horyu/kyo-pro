#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

const MAX_DIGIT: usize = 9;
const POW10: [usize; MAX_DIGIT] = {
    let mut arr = [0; MAX_DIGIT];
    let mut i = 0;
    while i < MAX_DIGIT {
        arr[i] = 10usize.pow(i as u32);
        i += 1;
    }
    arr
};

fn main() {
    input! {
        n: usize
    };
    let is753 = |vv: &[usize]| {
        let mut arr = [false; 8];
        for v in vv {
            arr[*v] = true;
        }
        arr[3] && arr[5] && arr[7]
    };
    let calc = |vv: &[usize]| {
        vv.iter()
            .zip(POW10.iter())
            .fold(0, |acc, (v, pow)| acc + v * pow)
    };

    let mut rs = 0;
    for digit in 3..=MAX_DIGIT {
        let iter = (0..digit).map(|_| [3usize, 5, 7]).multi_cartesian_product();
        for vv in iter {
            if is753(&vv) && calc(&vv) <= n {
                rs += 1;
            }
        }
    }
    println!("{}", rs);
}

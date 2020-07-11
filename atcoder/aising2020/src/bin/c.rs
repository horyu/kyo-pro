#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    for i in 1..=n {
        let mut vecs = vec![];
        // x >= y >= z
        for x in 1..=i {
            let xx = x * x;
            if xx >= i {
                continue;
            }
            for y in 1..=x {
                let yy = y * y;
                let xy = x * y;
                if (xx + yy + xy) >= i {
                    continue;
                }
                for z in 1..=y {
                    let sum = xx + yy + z * z + xy + y * z + z * x;
                    if sum == i {
                        vecs.push(vec![x, y, z]);
                    } else if sum > i {
                        break;
                    }
                }
            }
        }
        let count = vecs.iter().fold(0, |acc, vec| {
            if (vec[0] == vec[1]) && (vec[1] == vec[2]) {
                acc + 1
            } else if (vec[0] == vec[1]) || (vec[1] == vec[2]) {
                acc + 3
            } else {
                acc + 6
            }
        });
        println!("{}", count);
    }
}

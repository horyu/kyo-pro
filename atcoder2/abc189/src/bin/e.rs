#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        xxyy: [(isize, isize); n],
        m: usize,
    };
    let oopp = (0..m)
        .map(|_| {
            input! {op: isize};
            if 3 <= op {
                input! {p: isize};
                return (op, p);
            }
            (op, 0)
        })
        .chain([Default::default()])
        .collect_vec();
    input! {
        q: usize,
        aabb: [(usize, Usize1); q],
    };
    let a2iibb = multimap::MultiMap::<_, _>::from_iter(
        aabb.into_iter().enumerate().map(|(i, (a, b))| (a, (i, b))),
    );

    let mut rrss = vec![String::default(); q];
    // let mut mat = ndarray::Array2::from_diag(&ndarray::arr1(&[1isize; 3]));
    let mut mat = ndarray::array![[1, 0, 0], [0, 1, 0], [0, 0, 1],];
    // dbg!(mat.view());
    for (a, (o, p)) in oopp.into_iter().enumerate() {
        if let Some(iibb) = a2iibb.get_vec(&a) {
            for (i, b) in iibb.iter().copied() {
                // eprintln!("{a} {i} {b}");
                let (x, y) = xxyy[b];
                let point = mat.dot(&ndarray::arr1(&[x, y, 1]));
                rrss[i] = format!("{} {}", point[0], point[1]);
            }
        }
        // https://imagingsolution.net/imaging/affine-transformation/
        mat = match o {
            1 => ndarray::arr2(&[[0, 1, 0], [-1, 0, 0], [0, 0, 1]]).dot(&mat),
            2 => ndarray::arr2(&[[0, -1, 0], [1, 0, 0], [0, 0, 1]]).dot(&mat),
            3 => ndarray::arr2(&[[-1, 0, 2 * p], [0, 1, 0], [0, 0, 1]]).dot(&mat),
            4 => ndarray::arr2(&[[1, 0, 0], [0, -1, p * 2], [0, 0, 1]]).dot(&mat),
            _ => mat,
        };
        // eprintln!("{a} {:?}", mat.view());
    }
    let rs = rrss.iter().join("\n");
    println!("{rs}");

    if cfg!(debug_assertions) {
        let p = 5;
        assert!(
            ndarray::arr2(&[[-1, 0, 2 * p], [0, 1, 0], [0, 0, 1]])
                == ndarray::arr2(&[[1, 0, p], [0, 1, 0], [0, 0, 1]])
                    .dot(&ndarray::arr2(&[[-1, 0, 0], [0, 1, 0], [0, 0, 1]]))
                    .dot(&ndarray::arr2(&[[1, 0, -p], [0, 1, 0], [0, 0, 1]]))
        );
        assert!(
            ndarray::arr2(&[[1, 0, 0], [0, -1, p * 2], [0, 0, 1]])
                == ndarray::arr2(&[[1, 0, 0], [0, 1, p], [0, 0, 1]])
                    .dot(&ndarray::arr2(&[[1, 0, 0], [0, -1, 0], [0, 0, 1]]))
                    .dot(&ndarray::arr2(&[[1, 0, 0], [0, 1, -p], [0, 0, 1]]))
        )
    }
}

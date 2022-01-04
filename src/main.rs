use std::{
    collections::{HashMap, HashSet},
    result,
};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use rayon::{
    iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

fn main() {
    A346507();
    // let mut saved_results: HashMap<usize, usize> = HashMap::new();
    // saved_results.insert(0, 0);
    // for n in 1.. {
    //     let a = a34_up_to(10_usize.pow(n as u32));
    //     saved_results.insert(n, a);
    //     if !saved_results.contains_key(&(n - 1)) {
    //         let b = a34_up_to(10_usize.pow((n - 1) as u32));
    //         saved_results.insert(n - 1, b);
    //     }
    //     let result = a - saved_results.get(&(n - 1)).unwrap();
    //     println!("{}, {}", n, result);
    // }
}

fn A346507() {
    let r = helper(1000000000);

    for n in 1.. {
        let max = 10_u128.pow(n);
        let min = 10_u128.pow(n - 1);
        // println!("for {}, looking for > {} and < {}", n, min, max);
        let count_with_this = r.iter().filter(|n| **n >= min && **n <= max).count();
        println!("{} {}", n, count_with_this);
    }
    // println!("{:?}", r);

    fn helper(limit: u128) -> Vec<u128> {
        let a_limit = limit / 11 + 1;
        let a_steps = (11..a_limit).step_by(10).collect_vec();
        let groups = a_steps.chunks(60).map(|e| e.to_owned()).collect_vec();
        let mut result: Vec<u128> = groups.into_par_iter().map(|a_steps| {
            let mut local_result = FxHashSet::default();
            for a in a_steps {
                let b_limit = limit / a + 1;
                for b in (a..b_limit).step_by(10) {
                    local_result.insert(a * b);
                }
            }
            local_result
        }).flatten().collect();

        result.par_sort_unstable();
        result.dedup();
        result
    }
}

// fn a34_up_to(limit: usize) -> usize {

//     let a_limit = (limit / 11) + 1;
//     let a_steps: Vec<usize> = (11..a_limit).step_by(10).collect();
//     // println!("a setps: {}", a_steps.len());
//     let result: HashSet<usize> = a_steps.into_par_iter().map(|a| {
//         let b_limit = (limit / a) + 1;
//         let b_steps: Vec<usize> = (a..b_limit).step_by(10).collect();
//         (a, b_steps.into_par_iter())
//     })
//     .map(|(a, b_steps)| {
//         b_steps.map(move |b | a * b)
//     }).flatten().into_iter().unique();

//     result.len()

//     // result.par_sort_unstable();
//     // result.dedup();

//     // result.len()
// }

use std::{
    collections::{HashMap, HashSet},
    result,
};

use fxhash::FxHashSet;
use rayon::{iter::{IntoParallelRefIterator, IntoParallelIterator, ParallelIterator}, slice::ParallelSliceMut};

fn main() {
    let mut saved_results: HashMap<usize, usize> = HashMap::new();
    saved_results.insert(0, 0);
    for n in 1.. {
        let a = a34_up_to(10_usize.pow(n as u32));
        saved_results.insert(n, a);
        if !saved_results.contains_key(&(n - 1)) {
            let b = a34_up_to(10_usize.pow((n - 1) as u32));
            saved_results.insert(n - 1, b);
        }
        let result = a - saved_results.get(&(n - 1)).unwrap();
        println!("{}, {}", n, result);
    }
}

fn a34_up_to(limit: usize) -> usize {

    let a_limit = (limit / 11) + 1;
    let a_steps: Vec<usize> = (11..a_limit).step_by(10).collect();
    let result: FxHashSet<usize> = a_steps.into_par_iter().map(|a| {
        let mut result = FxHashSet::default();
        let b_limit = (limit / a) + 1;
        for b in (a..b_limit).step_by(10) {
            result.insert(a * b);
        }
        result
    }).flatten().collect();

    return result.len()
}
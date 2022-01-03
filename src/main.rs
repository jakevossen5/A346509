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
    // println!("a setps: {}", a_steps.len());
    let result: HashSet<usize> = a_steps.into_par_iter().map(|a| {
        let b_limit = (limit / a) + 1;
        let b_steps = (a..b_limit).step_by(10);
        (a, b_steps)
    })
    .filter(|(_, b)| b.len() != 0 )
    .map(|(a, b_steps)| {
        b_steps.map(move |y| (a, y)).collect::<Vec<(usize, usize)>>()
    }).flatten().map(|(a, b)| a * b).collect();

    result.len()

    // result.par_sort_unstable();
    // result.dedup();

    // result.len()
}
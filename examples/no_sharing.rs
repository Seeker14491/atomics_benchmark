#![feature(test)]

extern crate atomics_benchmark;

extern crate num_cpus;
extern crate rand;
extern crate test;
#[macro_use]
extern crate timeit;

use std::thread;
use rand::distributions::{IndependentSample, Range};

fn main() {
    let rand_range = Range::new(0, atomics_benchmark::ARRAY_SIZE);
    let num_threads = num_cpus::get();

    let mut arrays = Vec::with_capacity(num_threads);
    for _ in 0..num_threads {
        arrays.push(vec![0; atomics_benchmark::ARRAY_SIZE].into_boxed_slice());
    }

    let mut thread_handles = Vec::with_capacity(num_threads);

    println!("{} seconds", timeit_loops!(1, {
        for _ in 0..num_threads {
            let mut array = arrays.pop().unwrap();
            thread_handles.push(thread::spawn(move || {
                let mut rng = rand::weak_rng();
                for _ in 0..atomics_benchmark::ITERATIONS_PER_THREAD {
                    let random_index = rand_range.ind_sample(&mut rng);
                    array[random_index] += 1;
                }
                array
            }))
        }

        while let Some(handle) = thread_handles.pop() {
            test::black_box(handle.join().unwrap());
        }
    }));
}

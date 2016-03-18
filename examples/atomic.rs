#![feature(test)]

extern crate atomics_benchmark;

extern crate num_cpus;
extern crate rand;
extern crate test;
#[macro_use]
extern crate timeit;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use rand::distributions::{IndependentSample, Range};

fn main() {
    let rand_range = Range::new(0, atomics_benchmark::ARRAY_SIZE);
    let num_threads = num_cpus::get();

    let atomic_array = {
        let mut array = Vec::with_capacity(atomics_benchmark::ARRAY_SIZE);
        for _ in 0..atomics_benchmark::ARRAY_SIZE {
            array.push(AtomicUsize::new(0));
        }
        Arc::new(array.into_boxed_slice())
    };

    let mut thread_handles = Vec::with_capacity(num_threads);

    println!("{} seconds", timeit_loops!(1, {
        for _ in 0..num_threads {
            let atomic_array = atomic_array.clone();
            thread_handles.push(thread::spawn(move || {
                let mut rng = rand::weak_rng();
                for _ in 0..atomics_benchmark::ITERATIONS_PER_THREAD {
                    let random_index = rand_range.ind_sample(&mut rng);
                    atomic_array[random_index].store(atomic_array[random_index].load(Ordering::Relaxed) + 1, Ordering::Relaxed);
                }
            }))
        }

        while let Some(handle) = thread_handles.pop() {
            handle.join().unwrap();
        }
    }));

    test::black_box(&atomic_array);
}

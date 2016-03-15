#![feature(test)]

extern crate num_cpus;
extern crate rand;
extern crate test;

#[cfg(test)]
mod bench {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::thread;
    use num_cpus;
    use rand;
    use rand::distributions::{IndependentSample, Range};
    use test;
    use test::Bencher;

    const ARRAY_SIZE: usize = 100_000_000;

    #[bench]
    fn bench_atomic(bencher: &mut Bencher) {
        let rand_range = Range::new(0, ARRAY_SIZE);
        let num_threads = num_cpus::get();
        let stop_flag = Arc::new(AtomicBool::new(false));

        let atomic_array = {
            let mut array = Vec::with_capacity(ARRAY_SIZE);
            for _ in 0..ARRAY_SIZE {
                array.push(AtomicUsize::new(0));
            }
            Arc::new(array.into_boxed_slice())
        };

        let mut thread_handles = Vec::with_capacity(num_threads);

        for _ in 0..(num_threads - 1) {
            let atomic_array = atomic_array.clone();
            let stop_flag = stop_flag.clone();
            thread_handles.push(thread::spawn(move || {
                let mut rng = rand::weak_rng();
                let mut iteration_count = 0;
                while (iteration_count % 1_000_000 != 0) || (!stop_flag.load(Ordering::Relaxed)) {
                    let random_index = rand_range.ind_sample(&mut rng);
                    atomic_array[random_index].store(atomic_array[random_index].load(Ordering::Relaxed) + 1, Ordering::Relaxed);
                    iteration_count += 1;
                }
            }))
        }

        {
            let mut rng = rand::weak_rng();
            bencher.iter(|| {
                let random_index = rand_range.ind_sample(&mut rng);
                atomic_array[random_index].store(atomic_array[random_index].load(Ordering::Relaxed) + 1, Ordering::Relaxed);
            });
            stop_flag.store(true, Ordering::Relaxed);
        }

        while let Some(handle) = thread_handles.pop() {
            handle.join().unwrap();
        }

        test::black_box(&atomic_array);
    }
}

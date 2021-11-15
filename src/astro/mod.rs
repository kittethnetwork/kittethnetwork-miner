// AstroBWT Library

mod astrobwt;
mod salsa20;

// Async
use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;
use futures::future::{ready, pending};

// Threads
use std::time::Instant;
use std::thread;
const LENGTH: usize = 8;

struct Bench {
    bench: [u128; LENGTH],
    duration: [u128; LENGTH]
}

impl Bench {
    pub fn new() -> Bench {
        return Bench {
            bench: [0; 8],
            duration: [0; 8]
        };
    }
}

pub fn iters() {
    let iterations = 100;
    let mut benchmark = Bench::new();
    let mut index = 0;
    for bench in 1..=8 {
        let start = Instant::now();
        let mut handles = vec![];
        for _ in 0..bench {
            let handle = thread::spawn(move || {
                for _ in 0..iterations {
                    let random_bytes: Vec<u8> = (0..255).map(|_| { rand::random::<u8>() }).collect();
                    astrobwt::compute(&random_bytes, astrobwt::MAX_LENGTH);
                }
            });
            handles.push(handle);
        }

        for handle in handles { //wait on all threads
            handle.join().unwrap();
        }
        
        benchmark.bench[index] = bench;
        benchmark.duration[index] = start.elapsed().as_millis();
        index += 1;
    }

    println!("{:20} {:20} {:20} {:20} {:20}", "Threads", "Total Time", "Total Iterations", "Time/PoW (ms)", "Hash Rate/Sec");
    for i in 0..LENGTH {
        println!("{:20} {:20} {:20} {:20} {:20}", benchmark.bench[i], benchmark.duration[i], benchmark.bench[i]*iterations, benchmark.duration[i]/(benchmark.bench[i]*iterations), 1000f32 / (benchmark.duration[i] as f32 / (benchmark.bench[i]*iterations) as f32));
    }
}

pub fn iters_async() {
    let mut pool = LocalPool::new();
    let mut spawner = pool.spawner();

    let iterations = 100;
    let mut benchmark = Bench::new();
    let mut index = 0;
    for bench in 1..=8 {
        let start = Instant::now();
        for _ in 0..bench {
            for _ in 0..iterations {
                let future = async {
                    let random_bytes: Vec<u8> = (0..255).map(|_| { rand::random::<u8>() }).collect();
                    astrobwt::compute(&random_bytes, astrobwt::MAX_LENGTH);
                };
                spawner.spawn_local(future);
            }
        }

        pool.run();
        
        benchmark.bench[index] = bench;
        benchmark.duration[index] = start.elapsed().as_millis();
        index += 1;
    }

    println!("{:20} {:20} {:20} {:20} {:20}", "Threads", "Total Time", "Total Iterations", "Time/PoW (ms)", "Hash Rate/Sec");
    for i in 0..LENGTH {
        println!("{:20} {:20} {:20} {:20} {:20}", benchmark.bench[i], benchmark.duration[i], benchmark.bench[i]*iterations, benchmark.duration[i]/(benchmark.bench[i]*iterations), 1000f32 / (benchmark.duration[i] as f32 / (benchmark.bench[i]*iterations) as f32));
    }
}

mod version;
mod astro;

use std::time::Instant;

fn main() {
    println!("{}", version::get_version());
    let start = Instant::now();
    astro::iters();
    println!("{:?}", start.elapsed());
    let start = Instant::now();
    astro::iters_async();
    println!("{:?}", start.elapsed());
}

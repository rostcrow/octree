use crate::benchmark::run_benchmark;

mod octree;
mod benchmark;

fn main() {
    run_benchmark(1_000_000);
}

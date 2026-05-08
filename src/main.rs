use crate::benchmark::run_benchmark;
use crate::mountain::run_mountains;

mod octree;
mod benchmark;
mod mountain;

fn main() {
    let horizontal_line: &str = &"-".repeat(80);
    println!("{}", horizontal_line);
    println!("BENCHMARK");
    println!("{}", horizontal_line);
    run_benchmark(1_000_000);
    println!("{}", horizontal_line);

    println!("\n{}", horizontal_line);
    println!("MOUNTAINS");
    println!("{}", horizontal_line);
    run_mountains();
    println!("{}", horizontal_line);
}

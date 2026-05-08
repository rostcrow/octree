
use core::panic;
use std::ops::Range;

use crate::octree::{Location, OctreeDB, Point3D};
use rand::prelude::*;
use num_format::{ToFormattedString};

struct CoordinateRange {
    min: f64,
    max: f64,
}

#[derive(Debug, Clone)]
struct SimpleRecord {
    id: u64,
    position: Point3D,
}

impl SimpleRecord {
    fn new(id: u64, position: Point3D) -> Self {
        SimpleRecord { id, position }
    }
}

impl Location for SimpleRecord {
    fn point(&self) -> Point3D {
        self.position
    }
}

fn random_from_range(rng: &mut ThreadRng, range: &CoordinateRange) -> f64 {
    rng.random::<f64>() * (range.max - range.min) + range.min
}

fn random_point(rng: &mut ThreadRng, x_range: &CoordinateRange, y_range: &CoordinateRange, z_range: &CoordinateRange) -> Point3D {
    let x = random_from_range(rng, x_range);
    let y = random_from_range(rng, y_range);
    let z = random_from_range(rng, z_range);
    Point3D::new(x, y, z)
}

fn generate_random_records(n: usize, rng: &mut ThreadRng, x_range: &CoordinateRange, y_range: &CoordinateRange, z_range: &CoordinateRange) -> Vec<SimpleRecord> {
    let mut records = Vec::with_capacity(n);
    for i in 0..n {
        records.push(SimpleRecord::new(i as u64, random_point(rng, x_range, y_range, z_range)));
    }
    records
}

fn choose_random_record(rng: &mut ThreadRng, records: &[SimpleRecord]) -> SimpleRecord {
    let index = rng.random_range::<usize, Range<usize>>(0..records.len());
    records[index].clone()
}

pub fn run_benchmark(n_records: usize) {
    let format = num_format::CustomFormat::builder()
        .grouping(num_format::Grouping::Standard)
        .separator("_")
        .build()
        .unwrap();

    println!("GENERATING {} RANDOM RECORDS...", n_records.to_formatted_string(&format));
    let coord_range = CoordinateRange { min: 0.0, max: 100.0 };
    let mut rng = rand::rng();
    let records = generate_random_records(n_records, &mut rng, &coord_range, &coord_range, &coord_range);
    let records_clone = records.clone();
    println!("Generated");

    println!("\nOCTREE CREATION...");
    let now = std::time::Instant::now();
    let db = OctreeDB::from_data(records_clone).unwrap();
    let elapsed_creation = now.elapsed();
    println!("Octree created");
    println!("Octree records: {} records", db.n_records().to_formatted_string(&format));
    println!("Octree nodes  : {} nodes", db.octree_n_nodes().to_formatted_string(&format));
    println!("Octree height : {} levels", db.octree_height());
    println!("Octree size   : {} bytes", db.octree_size_bytes().to_formatted_string(&format));
    println!("Creation time : {:?} seconds", elapsed_creation.as_secs_f64());

    println!("\nFINDING BY POINT...");
    const N_POINT_QUERIES: usize = 100;
    let mut random_chosen = Vec::with_capacity(N_POINT_QUERIES);
    for _ in 0..N_POINT_QUERIES {
        let record = choose_random_record(&mut rng, &records);
        random_chosen.push(record.position);
    }
    let mut random_generated = Vec::with_capacity(N_POINT_QUERIES);
    for _ in 0..N_POINT_QUERIES {
        random_generated.push(random_point(&mut rng, &coord_range, &coord_range, &coord_range));
    }

    let mut total_success_time: f64 = 0.0;
    let mut total_n_sucess: u32 = 0;
    let mut total_fail_time: f64 = 0.0;
    let mut total_n_fail: u32 = 0;
    for query in random_chosen {
        let now = std::time::Instant::now();
        let result = db.find_by_point(&query);
        let elapsed = now.elapsed();
        if result.is_empty() {
            panic!("Expected to find a record for point {:?}, but found none", query);
        } else {
            total_success_time += elapsed.as_secs_f64();
            total_n_sucess += 1;
        }
    }

    for query in random_generated {
        let now = std::time::Instant::now();
        let result = db.find_by_point(&query);
        let elapsed = now.elapsed();
        if result.is_empty() {
            total_fail_time += elapsed.as_secs_f64();
            total_n_fail += 1;
        } else {
            total_success_time += elapsed.as_secs_f64();
            total_n_sucess += 1;
        }
    }

    println!("Avg. time for successful find by point: {:?} milliseconds ({} queries)", 1000.0 * total_success_time / total_n_sucess as f64, total_n_sucess);
    println!("Avg. time for failed find by point    : {:?} milliseconds ({} queries)", 1000.0 * total_fail_time / total_n_fail as f64, total_n_fail);

}

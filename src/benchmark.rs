
use crate::octree::{BoundingBox, Location, OctreeDB, Point3D};
use rand::prelude::*;
use num_format::{Locale, ToFormattedString};

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

fn generate_random_records(n: usize, x_range: &CoordinateRange, y_range: &CoordinateRange, z_range: &CoordinateRange) -> Vec<SimpleRecord> {
    let mut records = Vec::with_capacity(n);
    let mut rng = rand::rng();
    for i in 0..n {
        records.push(SimpleRecord::new(i as u64, random_point(&mut rng, x_range, y_range, z_range)));
    }
    records
}

pub fn run_benchmark(n_records: usize) {
    let format = num_format::CustomFormat::builder()
        .grouping(num_format::Grouping::Standard)
        .separator("_")
        .build()
        .unwrap();

    println!("GENERATING {} RANDOM RECORDS...", n_records.to_formatted_string(&format));
    let coord_range = CoordinateRange { min: 0.0, max: 100.0 };
    let records = generate_random_records(n_records, &coord_range, &coord_range, &coord_range);
    println!("Generated\n");

    println!("OCTREE CREATION...");
    let now = std::time::Instant::now();
    let db = OctreeDB::from_data(records).unwrap();
    let elapsed_creation = now.elapsed();
    println!("Octree created");
    println!("Octree records: {} records", db.n_records().to_formatted_string(&format));
    println!("Octree nodes  : {} nodes", db.octree_n_nodes().to_formatted_string(&format));
    println!("Octree height : {} levels", db.octree_height());
    println!("Octree size   : {} bytes", db.octree_size_bytes().to_formatted_string(&format));
    println!("Creation time : {:?} seconds", elapsed_creation.as_secs_f64());
}



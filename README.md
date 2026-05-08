# Octree

Implementation of octree database structure in Rust. 

This project was created for subject PDBS (Advanced database systems) at VŠB-TUO, Ostrava.

Author: Rostislav Vrána

## Running the project

Project can be run using standard cargo command `cargo run`.

## Project structure

- `octree.rs` - includes logic of the octree database structure with tests
- `benchmark.rs` - includes benchmarking and statistics about created octree database
- `mountain.rs` - uses octree database to query database of Czech mountains located in `data/hory_cr.json`
- `main.rs` - runs benchmark and mountain example

## OctreeDB usage example

Here is a simple usage example of OctreeDB

```rust
#[derive(Debug, Clone)]
struct Mountain {
    name: String,
    summit: Point3D,
}

impl Mountain {
    fn new(name: String, lat: f64, lon: f64, elev: f64) -> Self {
        Mountain {
            name,
            summit: Point3D::new(lat, lon, elev),
        }
    }
}

impl Location for Mountain {
    fn point(&self) -> Point3D {
        self.summit
    }
}

fn example() {
    let m1 = Mountain::new("Sněžka".to_string(), 50.736111, 15.739722, 1603.0);
    let m2 = Mountain::new("Praděd".to_string(), 50.083056, 17.230833, 1491.0);
    let db = OctreeDB::from_data(vec![m1, m2]).unwrap();
    let bounding_box = BoundingBox::new(
        Point3D::new(50.0, 15.0, 1500.0), 
        Point3D::new(51.0, 18.0, 1700.0)
    ).unwrap();
    let result = db.find_by_range(&bounding_box);
    println!("Found {} mountains", result.len()); // 1
    println!("{}", result[0].data.name);          // Sněžka
}
```
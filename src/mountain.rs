
use std::path::Path;

use crate::octree::{BoundingBox, Location, Point3D, Record, OctreeDB};

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

fn load_mountains(path: &Path) -> Result<Vec<Mountain>, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let json = serde_json::from_str::<serde_json::Value>(&content).map_err(|e| e.to_string())?;
    let mut res = Vec::new();
    if let Some(mountains) = json.as_array() {
        for mountain in mountains {
            let name = mountain.get("name").and_then(|v| v.as_str()).ok_or("Missing name")?.to_string();
            let lat = mountain.get("lat").and_then(|v| v.as_f64()).ok_or("Missing lat")?;
            let lon = mountain.get("lon").and_then(|v| v.as_f64()).ok_or("Missing lon")?;
            let elev = mountain.get("elev").and_then(|v| v.as_f64()).ok_or("Missing elev")?;
            res.push(Mountain::new(name, lat, lon, elev));
        }
    } else {
        return Err("Expected an array of mountains".to_string());
    }
    Ok(res)
}

fn print_query_result(result: Vec<Record<Mountain>>) {
    println!("Počet nalezených hor: {}", result.len());
    println!("{:<2} {:<25} {:<20} {:<20}", "#", "Název", "Nadmořská výška (m)", "Souřadnice (š, d)");
    println!("{}", "-".repeat(80));
    for (i, record) in result.iter().enumerate() {
        println!("{:<2} {:<25} {:<20} {:<20}", i + 1, record.data.name, record.data.summit.z, format!("({:.7}, {:.7})", record.data.summit.x, record.data.summit.y));
    }
}

pub fn run_mountains() {
    println!("LOADING MOUNTAINS...");
    let mountains = load_mountains(Path::new("./data/hory_cr.json")).expect("Failed to load mountains");
    println!("Loaded {} mountains", mountains.len());

    println!("\nCREATING OCTREE...");
    let octree = OctreeDB::from_data(mountains.clone()).expect("Failed to create octree");
    println!("Octree created");

    println!("\nQUERYING OCTREE...");
    println!("1. Všechny hory s nadmořskou výškou >= 1300 m (seřazené podle výšky):");
    let bounding_box1 = BoundingBox::new(Point3D::new(-90.0, -180.0, 1300.0), Point3D::new(90.0, 180.0, f64::INFINITY)).unwrap();
    let mut query1 = octree.find_by_range(&bounding_box1);
    query1.sort_by_key(|r| r.data.summit.z as u32);
    query1.reverse();
    print_query_result(query1);

    println!("\n2. Všechny hory s nadmořskou výškou >= 1100 m a pod (včetně) 49. rovnoběžkou (seřazené podle výšky):");
    let bounding_box2 = BoundingBox::new(Point3D::new(-90.0, -180.0, 1100.0), Point3D::new(49.0, 180.0, f64::INFINITY)).unwrap();
    let mut query2 = octree.find_by_range(&bounding_box2);
    query2.sort_by_key(|r| r.data.summit.z as u32);
    query2.reverse();
    print_query_result(query2);

    println!("\n3. Všechny hory mezi Opavou(S), Ostravou(V), Zlínem(J) a Olomoucí(Z) se nadmořskou výškou <800 m; 1000 m> (seřazené podle výšky):");
    let bounding_box3 = BoundingBox::new(Point3D::new(49.2300736, 17.2790949, 800.0), Point3D::new(49.9189497, 18.2373563, 1000.0)).unwrap();
    let mut query3 = octree.find_by_range(&bounding_box3);
    query3.sort_by_key(|r| r.data.summit.z as u32);
    query3.reverse();
    print_query_result(query3);
}
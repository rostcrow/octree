
use std::path::Path;

use crate::octree::{Location, Point3D};

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

pub fn run_mountains() {
    println!("LOADING MOUNTAINS...");
    let mountains = load_mountains(Path::new("./data/hory_cr.json")).expect("Failed to load mountains");
    println!("Loaded {} mountains", mountains.len());
}
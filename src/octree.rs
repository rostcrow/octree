
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }
}

struct BoundingBox {
    min: Point3D,
    max: Point3D,
}

impl BoundingBox {
    fn new(min: Point3D, max: Point3D) -> Self {
        BoundingBox { min, max }
    }

    fn contains(&self, point: &Point3D) -> bool {
        self.min.x <= point.x && point.x <= self.max.x &&
        self.min.y <= point.y && point.y <= self.max.y &&
        self.min.z <= point.z && point.z <= self.max.z
    }

    fn intersects(&self, other: &BoundingBox) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }

    fn split(&self) -> [BoundingBox; 8] {
        let mid_x = (self.min.x + self.max.x) / 2.0;
        let mid_y = (self.min.y + self.max.y) / 2.0;
        let mid_z = (self.min.z + self.max.z) / 2.0;

        [
            BoundingBox::new(Point3D::new(self.min.x, self.min.y, self.min.z), Point3D::new(mid_x, mid_y, mid_z)),
            BoundingBox::new(Point3D::new(mid_x, self.min.y, self.min.z), Point3D::new(self.max.x, mid_y, mid_z)),
            BoundingBox::new(Point3D::new(self.min.x, mid_y, self.min.z), Point3D::new(mid_x, self.max.y, mid_z)),
            BoundingBox::new(Point3D::new(mid_x, mid_y, self.min.z), Point3D::new(self.max.x, self.max.y, mid_z)),
            BoundingBox::new(Point3D::new(self.min.x, self.min.y, mid_z), Point3D::new(mid_x, mid_y, self.max.z)),
            BoundingBox::new(Point3D::new(mid_x, self.min.y, mid_z), Point3D::new(self.max.x, mid_y, self.max.z)),
            BoundingBox::new(Point3D::new(self.min.x, mid_y, mid_z), Point3D::new(mid_x, self.max.y, self.max.z)),
            BoundingBox::new(Point3D::new(mid_x, mid_y, mid_z), Point3D::new(self.max.x, self.max.y, self.max.z)),
        ]
    }
}

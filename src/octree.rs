use core::panic;

#[derive(Copy, Clone, PartialEq)]
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

#[derive(Copy, Clone)]
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

    fn region(self, point: &Point3D) -> Option<usize> {
        if !self.contains(point) {
            return None;
        }

        let mut index = 0;
        if point.x * 2.0 > (self.min.x + self.max.x) { index |= 1; }
        if point.y * 2.0 > (self.min.y + self.max.y) { index |= 2; }
        if point.z * 2.0 > (self.min.z + self.max.z) { index |= 4; }
        Some(index)
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

struct OctreeNode {
    bounding_box: BoundingBox,
    points: Vec<Point3D>,
    children: Vec<OctreeNode>,
}

impl OctreeNode {
    fn new_leaf(bounding_box: BoundingBox) -> Self {
        OctreeNode {
            bounding_box,
            points: Vec::new(),
            children: Vec::with_capacity(8),
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn height(&self) -> i32 {
        if self.is_leaf() {
            1
        } else {
            1 + self.children.iter().map(|child| child.height()).max().unwrap_or(0)
        }
    }

    fn n_points(&self) -> i32 {
        if self.is_leaf() {
            self.points.len() as i32
        } else {
            self.children.iter().map(|child| child.n_points()).sum()
        }
    }

    fn insert(&mut self, point: Point3D) -> bool {
        if !self.bounding_box.contains(&point) {
            // Point outside the bounding box cannot be inserted
            return false;
        }

        if self.is_leaf() {
            // Leaf node: insert point or subdivide if necessary
            if self.points.iter().any(|&p| p.ne(&point)) {
                // Subdivide the leaf node
                let children_boxes = self.bounding_box.split();
                for b in &children_boxes {
                    self.children.push(OctreeNode::new_leaf(*b));
                }
                
                self.points.push(point);
                for &p in &self.points {
                    if let Some(region) = self.bounding_box.region(&p) {
                        self.children[region].insert(p);
                    } else {
                        panic!("Point should be within the bounding box, but is not");
                    }
                }
                self.points.clear();
                true
            } else {
                // Empty leaf or duplicate point: just insert
                self.points.push(point);
                true
            }
        } else {
            // Internal node: delegate to the appropriate child
            let region = self.bounding_box.region(&point);
            if let Some(region) = region {
                self.children[region].insert(point)
            } else {
                panic!("Point should be within the bounding box, but is not");
            }
        }
    }
}

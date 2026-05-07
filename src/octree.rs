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

#[derive(Copy, Clone, PartialEq)]
struct RecordReference {
    point: Point3D,
    record_id: u64,
}

impl RecordReference {
    fn new(point: Point3D, record_id: u64) -> Self {
        RecordReference { point, record_id }
    }
}

#[derive(Copy, Clone)]
struct BoundingBox {
    min: Point3D,
    max: Point3D,
}

impl BoundingBox {
    fn new(min: Point3D, max: Point3D) -> Option<Self> {
        if min.x > max.x || min.y > max.y || min.z > max.z {
            None
        } else {
            Some(BoundingBox { min, max })
        }
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
        self.max.x >= other.min.x && self.min.x <= other.max.x &&
        self.max.y >= other.min.y && self.min.y <= other.max.y &&
        self.max.z >= other.min.z && self.min.z <= other.max.z
    }

    fn split(&self) -> [BoundingBox; 8] {
        let mid_x = (self.min.x + self.max.x) / 2.0;
        let mid_y = (self.min.y + self.max.y) / 2.0;
        let mid_z = (self.min.z + self.max.z) / 2.0;

        [
            BoundingBox::new(Point3D::new(self.min.x, self.min.y, self.min.z), Point3D::new(mid_x, mid_y, mid_z)).unwrap(),
            BoundingBox::new(Point3D::new(mid_x, self.min.y, self.min.z), Point3D::new(self.max.x, mid_y, mid_z)).unwrap(),
            BoundingBox::new(Point3D::new(self.min.x, mid_y, self.min.z), Point3D::new(mid_x, self.max.y, mid_z)).unwrap(),
            BoundingBox::new(Point3D::new(mid_x, mid_y, self.min.z), Point3D::new(self.max.x, self.max.y, mid_z)).unwrap(),
            BoundingBox::new(Point3D::new(self.min.x, self.min.y, mid_z), Point3D::new(mid_x, mid_y, self.max.z)).unwrap(),
            BoundingBox::new(Point3D::new(mid_x, self.min.y, mid_z), Point3D::new(self.max.x, mid_y, self.max.z)).unwrap(),
            BoundingBox::new(Point3D::new(self.min.x, mid_y, mid_z), Point3D::new(mid_x, self.max.y, self.max.z)).unwrap(),
            BoundingBox::new(Point3D::new(mid_x, mid_y, mid_z), Point3D::new(self.max.x, self.max.y, self.max.z)).unwrap(),
        ]
    }
}

struct OctreeNode {
    bounding_box: BoundingBox,
    references: Vec<RecordReference>,
    children: Vec<OctreeNode>,
}

impl OctreeNode {
    fn new_leaf(bounding_box: BoundingBox) -> Self {
        OctreeNode {
            bounding_box,
            references: Vec::new(),
            children: Vec::with_capacity(8),
        }
    }

    fn is_leaf(&self) -> bool {
        if self.children.is_empty() {
            true
        } else {
            if self.children.len() != 8 {
                panic!("Internal node must have exactly 8 children");
            }
            if !self.references.is_empty() {
                panic!("Internal node cannot have references");
            }
            false
        }
    }

    fn height(&self) -> i32 {
        if self.is_leaf() {
            1
        } else {
            1 + self.children.iter().map(|child| child.height()).max().unwrap_or(0)
        }
    }

    fn n_references(&self) -> u64 {
        if self.is_leaf() {
            self.references.len() as u64
        } else {
            self.children.iter().map(|child| child.n_references()).sum()
        }
    }

    fn insert(&mut self, point: Point3D, record_id: u64) -> bool {
        if !self.bounding_box.contains(&point) {
            // Point outside the bounding box cannot be inserted
            return false;
        }

        if self.is_leaf() {
            // Leaf node: insert point or subdivide if necessary
            if self.references.iter().any(|&r| r.point.ne(&point)) {
                // Subdivide the leaf node
                let children_boxes = self.bounding_box.split();
                for b in &children_boxes {
                    self.children.push(OctreeNode::new_leaf(*b));
                }
                
                self.references.push(RecordReference::new(point, record_id));
                for &r in &self.references {
                    if let Some(region) = self.bounding_box.region(&r.point) {
                        self.children[region].insert(r.point, r.record_id);
                    } else {
                        panic!("Point should be within the bounding box, but is not");
                    }
                }
                self.references.clear();
                true
            } else {
                // Empty leaf or duplicate point: just insert
                self.references.push(RecordReference::new(point, record_id));
                true
            }
        } else {
            // Internal node: delegate to the appropriate child
            let region = self.bounding_box.region(&point);
            if let Some(region) = region {
                self.children[region].insert(point, record_id)
            } else {
                panic!("Point should be within the bounding box, but is not");
            }
        }
    }

    fn find(&self, point: &Point3D) -> Vec<u64> {
        if !self.bounding_box.contains(point) {
            return Vec::new();
        }

        if self.is_leaf() {
            self.references.iter().filter(|&r| r.point == *point).map(|r| r.record_id).collect()
        } else {
            let region = self.bounding_box.region(point).unwrap();
            self.children[region].find(point)
        }
    }

}

struct Octree {
    root: OctreeNode,
}

impl Octree {
    fn new(bounding_box: BoundingBox) -> Self {
        Octree {
            root: OctreeNode::new_leaf(bounding_box),
        }
    }

    fn height(&self) -> i32 {
        self.root.height()
    }

    fn n_references(&self) -> u64 {
        self.root.n_references()
    }

    fn insert(&mut self, point: Point3D, record_id: u64) -> bool {
        self.root.insert(point, record_id)
    }

    fn find(&self, point: &Point3D) -> Vec<u64> {
        self.root.find(point)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bounding_box_new_correct() {
        let min = Point3D::new(0.0, 0.0, 0.0);
        let max = Point3D::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max);
        assert!(bbox.is_some());
    }

    #[test]
    fn bounding_box_new_incorrect() {
        let min = Point3D::new(1.0, 1.0, 1.0);
        let max = Point3D::new(0.0, 0.0, 0.0);
        let bbox = BoundingBox::new(min, max);
        assert!(bbox.is_none());
    }

    #[test]
    fn bounding_box_empty() {
        let min = Point3D::new(1.0, 1.0, 1.0);
        let max = Point3D::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max).unwrap();
        assert!(bbox.contains(&min));
        assert!(bbox.contains(&max));
    }

    #[test]
    fn bounding_box_contains() {
        let min = Point3D::new(0.0, 0.0, 0.0);
        let max = Point3D::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max).unwrap();
        assert!(bbox.contains(&Point3D::new(0.5, 0.5, 0.5)));
        assert!(!bbox.contains(&Point3D::new(-0.1, 0.5, 0.5)));
        assert!(bbox.contains(&min));
        assert!(bbox.contains(&max));
    }

    #[test]
    fn bounding_box_intersects() {
        let bbox1 = BoundingBox::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 1.0, 1.0)).unwrap();
        let bbox2 = BoundingBox::new(Point3D::new(0.5, 0.5, 0.5), Point3D::new(1.5, 1.5, 1.5)).unwrap();
        let bbox3 = BoundingBox::new(Point3D::new(1.1, 1.1, 1.1), Point3D::new(2.0, 2.0, 2.0)).unwrap();
        let bbox4 = BoundingBox::new(Point3D::new(1.0, 1.0, 1.0), Point3D::new(2.0, 2.0, 2.0)).unwrap();
        let bbox5 = BoundingBox::new(Point3D::new(1.0, 1.0, 1.0), Point3D::new(1.0, 1.0, 1.0)).unwrap();
        let bbox6 = BoundingBox::new(Point3D::new(-0.1, -0.1, -0.1), Point3D::new(0.0, 0.0, 0.0)).unwrap();
        assert!(bbox1.intersects(&bbox2));
        assert!(!bbox1.intersects(&bbox3));
        assert!(bbox1.intersects(&bbox1));
        assert!(bbox2.intersects(&bbox2));
        assert!(bbox3.intersects(&bbox3));
        assert!(bbox4.intersects(&bbox4));
        assert!(bbox1.intersects(&bbox4));
        assert!(bbox5.intersects(&bbox5));
        assert!(bbox6.intersects(&bbox6));
        assert!(bbox1.intersects(&bbox5));
        assert!(bbox1.intersects(&bbox6));
    }

    #[test]
    fn bounding_box_region() {
        let min = Point3D::new(0.0, 0.0, 0.0);
        let max = Point3D::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max).unwrap();
        assert_eq!(bbox.region(&Point3D::new(0.25, 0.25, 0.25)), Some(0));
        assert_eq!(bbox.region(&Point3D::new(-0.1, 0.5, 0.5)), None);
    }

    #[test]
    fn bounding_box_region_split_connection() {
        let min = Point3D::new(0.0, 0.0, 0.0);
        let max = Point3D::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max).unwrap();
        let children = bbox.split();
        let test_points = [
            Point3D::new(0.00, 0.00,0.00),
            Point3D::new(0.25, 0.25, 0.25),
            Point3D::new(0.75, 0.25, 0.25),
            Point3D::new(0.25, 0.75, 0.25),
            Point3D::new(0.75, 0.75, 0.25),
            Point3D::new(0.25, 0.25, 0.75),
            Point3D::new(0.75, 0.25, 0.75),
            Point3D::new(0.25, 0.75, 0.75),
            Point3D::new(0.75, 0.75, 0.75),
            Point3D::new(1.0, 1.0, 1.0),
        ];
        for point in &test_points {
            let region = bbox.region(point).unwrap();
            assert!(children[region].contains(point));
        }
    }

    #[test]
    fn octree_node_new_leaf() {
        let min = Point3D::new(0.0, 0.0, 0.0);
        let max = Point3D::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max).unwrap();
        let node = OctreeNode::new_leaf(bbox);
        assert!(node.is_leaf());
        assert_eq!(node.references.len(), 0);
        assert_eq!(node.children.len(), 0);
    }

    #[test]
    fn octree_node_is_leaf() {
        let min = Point3D::new(0.0, 0.0, 0.0);
        let max = Point3D::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max).unwrap();
        let mut node = OctreeNode::new_leaf(bbox);
        assert!(node.is_leaf());
        for _ in 0..8 {
            node.children.push(OctreeNode::new_leaf(bbox));
        }
        assert!(!node.is_leaf());
    }

    #[test]
    #[should_panic]
    fn octree_node_is_leaf_children_and_references() {
        let min = Point3D::new(0.0, 0.0, 0.0);
        let max = Point3D::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max).unwrap();
        let mut node = OctreeNode::new_leaf(bbox);
        node.children.push(OctreeNode::new_leaf(bbox));
        node.references.push(RecordReference::new(Point3D::new(0.5, 0.5, 0.5), 1));
        assert!(node.is_leaf());
    }

    #[test]
    #[should_panic]
    fn octree_node_is_leaf_wrong_number_of_children() {
        let min = Point3D::new(0.0, 0.0, 0.0);
        let max = Point3D::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max).unwrap();
        let mut node = OctreeNode::new_leaf(bbox);
        for _ in 0..7 {
            node.children.push(OctreeNode::new_leaf(bbox));
        }
        assert!(!node.is_leaf());
    }

    #[test]
    fn octree_node_height() {
        let min = Point3D::new(0.0, 0.0, 0.0);
        let max = Point3D::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max).unwrap();
        let mut node = OctreeNode::new_leaf(bbox);
        assert_eq!(node.height(), 1);
        for _ in 0..8 {
            node.children.push(OctreeNode::new_leaf(bbox));
        }
        assert_eq!(node.height(), 2);
    }

    #[test]
    fn octree_node_n_references() {
        let min = Point3D::new(0.0, 0.0, 0.0);
        let max = Point3D::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max).unwrap();
        let mut node = OctreeNode::new_leaf(bbox);
        assert_eq!(node.n_references(), 0);
        node.references.push(RecordReference::new(Point3D::new(0.5, 0.5, 0.5), 1));
        assert_eq!(node.n_references(), 1);
        node.references.pop();
        for _ in 0..8 {
            node.children.push(OctreeNode::new_leaf(bbox));
        }
        assert_eq!(node.n_references(), 0);
        node.children[0].references.push(RecordReference::new(Point3D::new(0.25, 0.25, 0.25), 2));
        assert_eq!(node.n_references(), 1);
    }

    #[test]
    fn octree_node_insert_outside() {
        let min = Point3D::new(0.0, 0.0, 0.0);
        let max = Point3D::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max).unwrap();
        let mut node = OctreeNode::new_leaf(bbox);
        assert!(!node.insert(Point3D::new(-0.1, 0.5, 0.5), 1));
    }

    #[test]
    fn octree_node_insert_leaf() {
        let min = Point3D::new(0.0, 0.0, 0.0);
        let max = Point3D::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max).unwrap();
        let mut node = OctreeNode::new_leaf(bbox);
        assert!(node.insert(Point3D::new(0.5, 0.5, 0.5), 1));
        assert!(node.is_leaf());
        assert_eq!(node.n_references(), 1);
        assert_eq!(node.height(), 1);
    }

    #[test]
    fn octree_node_insert_subdivide() {
        let min = Point3D::new(0.0, 0.0, 0.0);
        let max = Point3D::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max).unwrap();
        let mut node = OctreeNode::new_leaf(bbox);
        for i in 0..10 {
            assert!(node.insert(Point3D::new(0.1 * i as f64, 0.1 * i as f64, 0.1 * i as f64), i));
        }
        assert!(!node.is_leaf());
        assert_eq!(node.n_references(), 10);
        assert_eq!(node.height(), 5);
    }

    #[test]
    fn octree_node_insert_duplicate() {
        let min = Point3D::new(0.0, 0.0, 0.0);
        let max = Point3D::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max).unwrap();
        let mut node = OctreeNode::new_leaf(bbox);
        assert!(node.insert(Point3D::new(0.5, 0.5, 0.5), 1));
        assert!(node.insert(Point3D::new(0.5, 0.5, 0.5), 2));
        assert!(node.is_leaf());
        assert_eq!(node.n_references(), 2);
        assert_eq!(node.height(), 1);
    }

    #[test]
    fn octree_node_find() {
        let min = Point3D::new(0.0, 0.0, 0.0);
        let max = Point3D::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max).unwrap();
        let mut node = OctreeNode::new_leaf(bbox);
        for i in 0..10 {
            assert!(node.insert(Point3D::new(0.1 * i as f64, 0.1 * i as f64, 0.1 * i as f64), i));
        }
        assert!(node.insert(Point3D::new(0.5, 0.5, 0.5), 10));

        assert_eq!(node.find(&Point3D::new(0.5, 0.5, 0.5)), vec![5, 10]);
        assert_eq!(node.find(&Point3D::new(0.1, 0.1, 0.1)), vec![1]);
        assert_eq!(node.find(&Point3D::new(0.9, 0.9, 0.9)), vec![9]);
        assert_eq!(node.find(&Point3D::new(0.0, 0.0, 0.0)), vec![0]);
        assert_eq!(node.find(&Point3D::new(0.15, 0.15, 0.15)), Vec::<u64>::new());
        assert_eq!(node.find(&Point3D::new(1.1, 1.1, 1.1)), Vec::<u64>::new());
    }

}
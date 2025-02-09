struct Point {
    lat: f64, lon: f64,
}

struct Bound {
    min_lat: f64, max_lat: f64,
    min_lon: f64, max_lon: f64,
}

impl Bound {
    fn from_min_max(min: Point, max: Point) -> Self {
        Bound {
            min_lat: min.lat, min_lon: min.lon,
            max_lat: max.lat, max_lon: max.lon,
        }
    }
}

struct LeafNode {
    id: usize,
    point: Point,
}

struct InternalNode {
    bound: Bound,
    childs: Vec<Node>,
}

enum Node {
    Leaf(Vec<LeafNode>),
    Internal(InternalNode),
}

struct RTree {
    order: usize,
    root: Option<Node>,
    max_area: Bound,
}

impl RTree {
    fn new(order: usize, min: Point, max: Point ) -> Self {
        RTree {
            root: None,
            max_area: Bound::from_min_max(min, max),
            order,
        }
    }
}
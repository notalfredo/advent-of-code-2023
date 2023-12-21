#[derive(Eq, Hash, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct Node {
    pub loc: Point,
    pub weight: u32,
}

impl Node {
    pub fn new(loc: Point, weight: u32) -> Self {
        Self { loc, weight }
    }
}

pub struct PriorityQueue<'a> {
    pub queue: Vec<(&'a Node, u32)>,
}

impl<'a> PriorityQueue<'a> {    
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }
    pub fn insert(&mut self, node: &'a Node, cost: u32) {
        self.queue.push((node, cost));
        self.queue.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());
    }
    pub fn is_empty(&self) -> bool {
        self.queue.len() == 0 
    }
    pub fn pop(&mut self) -> (&'a Node, u32) {
        self.queue.remove(0)
    }

}

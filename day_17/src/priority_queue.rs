use crate::Graph;


#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Direction {
    North(u32),
    South(u32),
    East(u32),
    West(u32)
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
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
    pub queue: Vec<(&'a Node, Direction, u32)>,
}

impl<'a> PriorityQueue<'a> {    
    pub fn new(
            right: (&'a Node, Direction, u32),
            bottom: (&'a Node, Direction, u32)
        ) -> Self {
        let queue: Vec<(&'a Node, Direction, u32)> = vec![right, bottom];
        let mut temp = Self { queue };
        temp.sort();
        temp
    }
    pub fn dump(&self) {
        println!("Current queue: ");
        for node in &self.queue {
            println!("      {:?}", node);
        }
    }
    fn sort(&mut self) {
        self.queue.sort_by(|a, b| (a.2).partial_cmp(&b.2).unwrap());
    }

    pub fn insert(&mut self, node: &'a Node, dir: Direction, cost: u32) {
        self.queue.push((node, dir, cost));
        self.sort();
        self.dump();
    }
    pub fn is_empty(&self) -> bool {
        self.queue.len() == 0 
    }
    pub fn pop(&mut self) -> (&'a Node, Direction, u32) {
        self.queue.remove(0)
    }
}

impl Graph {
    pub fn get_height(&self) -> usize {
        self.map.len()
    }

    pub fn get_width(&self) -> usize {
        self.map[0].len()
    }

    pub fn is_goal(&self, node: &Node) -> bool {
        (node.loc.x as usize == self.get_width() - 1) 
            &&
        (node.loc.y as usize == self.get_height() - 1) 
    }

    pub fn within_bounds(&self, x: isize, y: isize) -> bool {
        ((x >= 0) && (x <= (self.get_width() - 1) as isize))
            &&
        ((y >= 0) && (y <= (self.get_height() - 1) as isize))
    }

    pub fn gen_neighbor(&self, x: isize, y: isize) -> Option<&Node> {
        if self.within_bounds(x, y) {
            return Some(&self.map[y as usize][x as usize]);
        }
        None
    }

    pub fn gen_west<'a>(&'a self, neighbors: &mut Vec<(&'a Node, Direction)>, node: &Node, direction: Direction) {
        if let Some(found) = self.gen_neighbor(node.loc.x as isize - 1, node.loc.y as isize) {
            neighbors.push((found, direction));
        }
    }

    pub fn gen_east<'a>(&'a self, neighbors: &mut Vec<(&'a Node, Direction)>, node: &Node, direction: Direction) {
        if let Some(found) = self.gen_neighbor(node.loc.x as isize + 1, node.loc.y as isize) {
            neighbors.push((found, direction));
        }
    }

    pub fn gen_north<'a>(&'a self, neighbors: &mut Vec<(&'a Node, Direction)>, node: &Node, direction: Direction) {
        if let Some(found) = self.gen_neighbor(node.loc.x as isize, node.loc.y as isize - 1) {
            neighbors.push((found, direction));
        }
    }

    pub fn gen_south<'a>(&'a self, neighbors: &mut Vec<(&'a Node, Direction)>, node: &Node, direction: Direction) {
        if let Some(found) = self.gen_neighbor(node.loc.x as isize, node.loc.y as isize + 1) {
            neighbors.push((found, direction));
        }
    }
}

mod priority_queue;
use std::{collections::HashMap, hash::Hash};

use priority_queue::{PriorityQueue, Node, Point};

struct Graph {
    map: Vec<Vec<Node>>,
}

impl Graph {
    fn new(file: &str) -> Self {
        let map = file
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        Node::new(
                            Point::new(x as u32, y as u32), 
                            c.to_digit(10).unwrap()    
                        )
                    })
                    .collect::<Vec<Node>>()
            })
            .collect::<Vec<_>>();
        Self { map }
    }

    fn get_height(&self) -> usize {
        self.map.len()
    }

    fn get_width(&self) -> usize {
        self.map[0].len()
    }

    fn is_goal(&self, node: &Node) -> bool {
        (node.loc.x as usize == self.get_width() - 1) 
            &&
        (node.loc.y as usize == self.get_height() - 1) 
    }

    fn within_bounds(&self, x: isize, y: isize) -> bool {
        ((x >= 0) && (x <= (self.get_width() - 1) as isize))
            &&
        ((y >= 0) && (y <= (self.get_height() - 1) as isize))
    }

    fn neighbors(&self, node: &Node) -> Vec<&Node> {
        let mut neighbors: Vec<&Node> = Vec::new();

        //Top neighbor
        if self.within_bounds(node.loc.x as isize, node.loc.y as isize - 1) {
            neighbors.push(&self.map[node.loc.x as usize][(node.loc.y - 1) as usize]);
        }
        //Bottom neighbor
        if self.within_bounds(node.loc.x as isize, node.loc.y as isize + 1) {
            neighbors.push(&self.map[node.loc.x as usize][(node.loc.y + 1) as usize]);
        }
        //left neighbor
        if self.within_bounds(node.loc.x as isize - 1, node.loc.y as isize) {
            neighbors.push(&self.map[(node.loc.x - 1) as usize][node.loc.y as usize]);
        }
        //right neighbor
        if self.within_bounds(node.loc.x as isize + 1, node.loc.y as isize) {
            neighbors.push(&self.map[(node.loc.x + 1) as usize][node.loc.y as usize]);
        }

        neighbors
    }  


    fn q1(&self) {
        let mut fringe = PriorityQueue::new();
        fringe.insert(&self.map[0][0], 0);

        let mut came_from: HashMap<&Node, Option<&Node>> = HashMap::new(); 
        let mut cost_so_far: HashMap<&Node, u32> = HashMap::new();

        came_from.insert(&self.map[0][0], None); 
        cost_so_far.insert(&self.map[0][0], 0); 

        
        while !fringe.is_empty() {
            let (current, cost_from_current): (&Node, u32) = fringe.pop();


            if self.is_goal(current){
                break;
            }

            
            for next in self.neighbors(current).iter() {
                let new_cost = cost_so_far[current] + next.weight;
                if !(cost_so_far.contains_key(next)) || (new_cost < *cost_so_far.get(next).unwrap()) {
                    match cost_so_far.get_mut(next) {
                        Some(next_node) => {
                            *next_node = new_cost;
                        },
                        None => {
                            cost_so_far.insert(next, new_cost);
                        }
                    }
                    fringe.insert(next, new_cost);
                    match came_from.get_mut(next) {
                        Some(next_node) => {
                            *next_node = Some(current);
                        },
                        None => {
                            came_from.insert(next, Some(current));
                        }
                    }
                }
            }
        }

        println!("{:?}", cost_so_far.get(
                            &self.map[self.get_height()-1][self.get_width()-1]
            ).unwrap()
        );
    }
}

fn main() {
    let file = include_str!("../input/sample.txt");
    let graph = Graph::new(file);
    graph.q1();
}

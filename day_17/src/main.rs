mod priority_queue;
use std::{collections::HashMap, hash::Hash};

use priority_queue::{PriorityQueue, Node, Direction, Point};
use priority_queue::Direction::{*};

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

    fn neighbors(&self, node: &Node, direction: Direction) -> Vec<(&Node, Direction)> {
        let mut neighbors: Vec<(&Node, Direction)> = Vec::new();

        match direction {
            North(score) | South(score) => {

                self.gen_west(&mut neighbors, node, West(2));
                self.gen_east(&mut neighbors, node, East(2));

                if score != 0 {
                    match direction {
                        North(_) => self.gen_north(&mut neighbors, node, North(score - 1)),
                        South(_) => self.gen_south(&mut neighbors, node, South(score - 1)),
                        _ => panic!("Matched west/east inside a north/south"),
                    }
                }
            },
            East(score) | West(score) => {
                self.gen_north(&mut neighbors, node, North(2));
                self.gen_south(&mut neighbors, node, South(2));
                if score != 0 {
                    match direction {
                        East(_) => self.gen_east(&mut neighbors, node, East(score - 1)),
                        West(_) => self.gen_west(&mut neighbors, node, West(score - 1)),
                        _ => panic!("Matched north/south inside a east/west"),
                    }
                }
            },
        }
        neighbors
    }  
    
    fn reconstruct_path(&self, came_from: HashMap<&Node, &Node>) {
        let mut current = &self.map[self.get_height()-1][self.get_width()-1]; 
        let mut path: Vec<&Node> = Vec::new();

        while (current.loc.x != 0) || (current.loc.y != 0) {
            path.push(current);
            current = came_from.get(current).unwrap();
        }
        let path: Vec<&Node> = path.into_iter().rev().collect();
        
        for node in path.iter() {
            println!("{:?}", node);
        }

    }
    fn get_cost(&self, cost: HashMap<(&Node, Direction), u32>) {
        
    }

    fn q1(&self) {
        let mut fringe = PriorityQueue::new(
            (&self.map[0][1], East(2),  self.map[0][1].weight),
            (&self.map[1][0], South(2), self.map[1][0].weight),
        );
        let mut came_from: HashMap<(&Node, Direction), (&Node, Direction)> = HashMap::from([
             ((&self.map[0][1], East(2)), (&self.map[0][0],North(2))),
             ((&self.map[1][0], South(2)), (&self.map[0][0], North(2))),
        ]); 
        let mut cost_so_far: HashMap<(&Node, Direction), u32> = HashMap::from([
             ((&self.map[0][1], East(2)), self.map[0][1].weight),
             ((&self.map[1][0], South(2)), self.map[1][0].weight),
        ]);
        let mut total_cost: u32 = 0;

        
        while !fringe.is_empty() {
            let (current, direction, cost_from_current): (&Node, Direction, u32) = fringe.pop();
            println!("Current node, {:?}, direction {:?}", current, direction);


            if self.is_goal(current){
                total_cost = cost_from_current;
                break;
            }

            for (next, next_direction) in self.neighbors(current, direction).iter() {
                println!("     Looking at the neighbor, {:?}, direction {:?}", next, next_direction);
                let new_cost = cost_so_far[&(current, direction)] + next.weight;
                if !(cost_so_far.contains_key(&((next, *next_direction)))) 
                     || (new_cost < *cost_so_far.get(&((next, *next_direction))).unwrap()) {
                    println!("          |");
                    println!("          |-> passed, new_cost, {:}", new_cost);
                    match cost_so_far.get_mut(&((next, *next_direction))) {
                        Some(next_node) => {
                            *next_node = new_cost;
                        },
                        None => {
                            cost_so_far.insert((next, *next_direction), new_cost);
                        }
                    }
                    fringe.insert(next, *next_direction, new_cost);
                    match came_from.get_mut(&((next, *next_direction))) {
                        Some(next_node) => {
                            *next_node = (current, direction);
                        },
                        None => {
                            came_from.insert((next, *next_direction), (current, direction));
                        }
                    }
                }
            }
        }

        println!("==================");
        println!("==================");
        println!("==================");

        println!("Q1 Dist {:?}", total_cost)

        //self.reconstruct_path(came_from);
        //for test in came_from.iter() {
        //    println!("{:?}", test);
        //}
        //println!("============");
        //for test in cost_so_far.iter() {
        //    println!("{:?}", test);
        //}
    }
}

fn main() {
    //let file = include_str!("../input/sample.txt");
    //let file = include_str!("../input/sample_two.txt");
    let file = include_str!("../input/input.txt");
    let graph = Graph::new(file);
    graph.q1();
}

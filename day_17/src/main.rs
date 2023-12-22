mod priority_queue;
use std::{collections::HashMap, hash::Hash};

use priority_queue::Direction::*;
use priority_queue::{Direction, Node, Point, PriorityQueue};

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
                        Node::new(Point::new(x as u32, y as u32), c.to_digit(10).unwrap())
                    })
                    .collect::<Vec<Node>>()
            })
            .collect::<Vec<_>>();
        Self { map }
    }

    fn neighbors(
        &self,
        node: &Node,
        direction: Direction,
        forward_dist: u32,
        ultra_crucibles: bool,
    ) -> Vec<(&Node, Direction)> {
        let mut neighbors: Vec<(&Node, Direction)> = Vec::new();

        match direction {
            North(score) | South(score) => {
                //We have to move straight a minimum of four blocks
                if ultra_crucibles && (score > 6) {
                    match direction {
                        North(_) => self.gen_north(&mut neighbors, node, North(score - 1)),
                        South(_) => self.gen_south(&mut neighbors, node, South(score - 1)),
                        _ => panic!("Matched west/east inside a north/south"),
                    }
                } else {
                    self.gen_west(&mut neighbors, node, West(forward_dist));
                    self.gen_east(&mut neighbors, node, East(forward_dist));

                    if score != 0 {
                        match direction {
                            North(_) => self.gen_north(&mut neighbors, node, North(score - 1)),
                            South(_) => self.gen_south(&mut neighbors, node, South(score - 1)),
                            _ => panic!("Matched west/east inside a north/south"),
                        }
                    }
                }
                return neighbors;
            }
            East(score) | West(score) => {
                //We have to move straight a minimum of four blocks
                if ultra_crucibles && (score > 6) {
                    match direction {
                        East(_) => self.gen_east(&mut neighbors, node, East(score - 1)),
                        West(_) => self.gen_west(&mut neighbors, node, West(score - 1)),
                        _ => panic!("Matched north/south inside a east/west"),
                    }
                } else {
                    self.gen_north(&mut neighbors, node, North(forward_dist));
                    self.gen_south(&mut neighbors, node, South(forward_dist));
                    if score != 0 {
                        match direction {
                            East(_) => self.gen_east(&mut neighbors, node, East(score - 1)),
                            West(_) => self.gen_west(&mut neighbors, node, West(score - 1)),
                            _ => panic!("Matched north/south inside a east/west"),
                        }
                    }
                }
                return neighbors;
            }
        }
    }

    /*
     * foward_dist: how for ahead a node
     *              is allowed to travel
     */
    fn find_path(&self, forward_dist: u32, ultra_crucibles: bool) -> u32 {
        let mut fringe = PriorityQueue::new(
            (&self.map[0][1], East(forward_dist), self.map[0][1].weight),
            (&self.map[1][0], South(forward_dist), self.map[1][0].weight),
        );
        let mut came_from: HashMap<(&Node, Direction), (&Node, Direction)> = HashMap::from([
            (
                (&self.map[0][1], East(forward_dist)),
                (&self.map[0][0], North(forward_dist)),
            ),
            (
                (&self.map[1][0], South(forward_dist)),
                (&self.map[0][0], North(forward_dist)),
            ),
        ]);
        let mut cost_so_far: HashMap<(&Node, Direction), u32> = HashMap::from([
            ((&self.map[0][1], East(forward_dist)), self.map[0][1].weight),
            (
                (&self.map[1][0], South(forward_dist)),
                self.map[1][0].weight,
            ),
        ]);
        let mut total_cost: u32 = 0;

        while !fringe.is_empty() {
            let (current, direction, cost_from_current): (&Node, Direction, u32) = fringe.pop();

            if self.is_goal(current) {
                if ultra_crucibles {
                    match direction {
                        North(val) | South(val) | West(val) | East(val) => {
                            if val > 6 {
                                continue;
                            }
                        }
                    }
                }
                total_cost = cost_from_current;
                break;
            }

            for (next, next_direction) in self
                .neighbors(current, direction, forward_dist, ultra_crucibles)
                .iter()
            {
                let new_cost = cost_so_far[&(current, direction)] + next.weight;
                if !(cost_so_far.contains_key(&((next, *next_direction))))
                    || (new_cost < *cost_so_far.get(&((next, *next_direction))).unwrap())
                {
                    match cost_so_far.get_mut(&((next, *next_direction))) {
                        Some(next_node) => {
                            *next_node = new_cost;
                        }
                        None => {
                            cost_so_far.insert((next, *next_direction), new_cost);
                        }
                    }
                    fringe.insert(next, *next_direction, new_cost);
                    match came_from.get_mut(&((next, *next_direction))) {
                        Some(next_node) => {
                            *next_node = (current, direction);
                        }
                        None => {
                            came_from.insert((next, *next_direction), (current, direction));
                        }
                    }
                }
            }
        }

        total_cost
    }

    fn q1(&self) -> u32 {
        self.find_path(2, false)
    }

    fn q2(&self) -> u32 {
        self.find_path(9, true)
    }
}

fn main() {
    //let file = include_str!("../input/sample.txt");
    //let file = include_str!("../input/sample_two.txt");
    //let file = include_str!("../input/sample_three.txt");
    let file = include_str!("../input/input.txt");
    let graph = Graph::new(file);
    println!("Q1: {:}", graph.q1());
    println!("Q2: {:}", graph.q2());
}

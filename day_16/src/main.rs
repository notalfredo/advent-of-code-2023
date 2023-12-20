use std::cmp;
use std::{collections::HashMap, hash::Hash};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Light {
    direction: Direction,
    loc: Point,
}

impl Light {
    fn new(direction: Direction, loc: Point) -> Self {
        Self { direction, loc }
    }
    fn get_next_pos(&self) -> Point {
        match self.direction {
            Direction::North => Point::new(self.loc.x, self.loc.y - 1),
            Direction::South => Point::new(self.loc.x, self.loc.y + 1),
            Direction::West => Point::new(self.loc.x - 1, self.loc.y),
            Direction::East => Point::new(self.loc.x + 1, self.loc.y),
        }
    }
    fn keep_same_dir(&self, point: Point) -> Point {
        match self.direction {
            Direction::North => Point::new(point.x, point.y - 1),
            Direction::South => Point::new(point.x, point.y + 1),
            Direction::West => Point::new(point.x - 1, point.y),
            Direction::East => Point::new(point.x + 1, point.y),
        }
    }

    fn split_light(&self, point: Point) -> (Point, Point) {
        match self.direction {
            Direction::North | Direction::South => (
                Point::new(point.x - 1, point.y),
                Point::new(point.x + 1, point.y),
            ),
            Direction::West | Direction::East => (
                Point::new(point.x, point.y - 1),
                Point::new(point.x, point.y + 1),
            ),
        }
    }

    fn diag_left(&self, point: Point) -> (Direction, Point) {
        match self.direction {
            Direction::North => (Direction::West, Point::new(point.x - 1, point.y)),
            Direction::South => (Direction::East, Point::new(point.x + 1, point.y)),
            Direction::West => (Direction::North, Point::new(point.x, point.y - 1)),
            Direction::East => (Direction::South, Point::new(point.x, point.y + 1)),
        }
    }

    fn diag_right(&self, point: Point) -> (Direction, Point) {
        match self.direction {
            Direction::North => (Direction::East, Point::new(point.x + 1, point.y)),
            Direction::South => (Direction::West, Point::new(point.x - 1, point.y)),
            Direction::West => (Direction::South, Point::new(point.x, point.y + 1)),
            Direction::East => (Direction::North, Point::new(point.x, point.y - 1)),
        }
    }

    fn new_lights(self, curr_tile: Tile) -> (Light, Option<Light>) {
        match curr_tile {
            Tile::Empty => {
                return (
                    Light::new(self.direction, self.keep_same_dir(self.loc)),
                    None,
                );
            }
            Tile::Vertical => match self.direction {
                Direction::North | Direction::South => {
                    return (
                        Light::new(self.direction, self.keep_same_dir(self.loc)),
                        None,
                    );
                }
                Direction::West | Direction::East => {
                    let (north, south) = self.split_light(self.loc);
                    return (
                        Light::new(Direction::North, north),
                        Some(Light::new(Direction::South, south)),
                    );
                }
            },
            Tile::Horizontal => match self.direction {
                Direction::North | Direction::South => {
                    let (west, east) = self.split_light(self.loc);
                    return (
                        Light::new(Direction::West, west),
                        Some(Light::new(Direction::East, east)),
                    );
                }
                Direction::West | Direction::East => {
                    return (
                        Light::new(self.direction, self.keep_same_dir(self.loc)),
                        None,
                    );
                }
            },
            Tile::MirrorL => match self.direction {
                _ => {
                    let (dir, loc) = self.diag_left(self.loc);
                    return (Light::new(dir, loc), None);
                }
            },
            Tile::MirrorR => match self.direction {
                _ => {
                    let (dir, loc) = self.diag_right(self.loc);
                    return (Light::new(dir, loc), None);
                }
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Tile {
    Empty,      // '.'
    Vertical,   // '|'
    Horizontal, // '-'
    MirrorL,    // '\'
    MirrorR,    // '/'
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            '/' => Tile::MirrorR,
            '\\' => Tile::MirrorL,
            _ => panic!("Unkown Symbol"),
        }
    }
}

struct Map {
    map: Vec<Vec<Tile>>,
    energized: HashMap<Point, Direction>,
    fringe: Vec<Light>,
}

impl Map {
    fn new(file: &str) -> Self {
        let map: Vec<Vec<Tile>> = file
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| Tile::from(c))
                    .collect::<Vec<Tile>>()
            })
            .collect();
        let energized: HashMap<Point, Direction> = HashMap::new();
        let fringe: Vec<Light> = Vec::new();
        Self {
            map,
            energized,
            fringe,
        }
    }
    fn get_height(&self) -> usize {
        self.map.len()
    }
    fn get_width(&self) -> usize {
        self.map[0].len()
    }

    fn within_bound(&self, point: Point) -> bool {
        ((point.x >= 0) && ((point.x as usize) < self.get_width()))
            && ((point.y >= 0) && ((point.y as usize) < self.get_height()))
    }

    fn energize(&mut self) -> usize {
        while self.fringe.len() > 0 {
            let curr_light: Light = self.fringe.pop().unwrap();
            let next_tile = self.map[curr_light.loc.y as usize][curr_light.loc.x as usize];

            match self.energized.get(&curr_light.loc) {
                Some(dir) => {
                    if *dir == curr_light.direction {
                        continue;
                    }
                }
                None => {
                    self.energized.insert(curr_light.loc, curr_light.direction);
                }
            }

            let (next_light, extra_light) = curr_light.new_lights(next_tile);

            if self.within_bound(next_light.loc) {
                self.fringe.push(next_light);
            }

            if let Some(extra_light) = extra_light {
                if self.within_bound(extra_light.loc) {
                    self.fringe.push(extra_light);
                }
            }
        }

        self.energized.len()
    }

    fn q1(&mut self) -> usize {
        self.fringe = vec![Light::new(Direction::East, Point::new(0, 0))];
        self.energize()
    }

    fn q2(&mut self) -> usize {
        let left_max = (0..self.get_width())
            .map(|y| {
                self.fringe = vec![Light::new(Direction::East, Point::new(0, y as i32))];
                self.energized = HashMap::new();
                self.energize()
            })
            .max()
            .unwrap();

        let right_max = (0..self.get_width())
            .map(|y| {
                self.fringe = vec![Light::new(
                    Direction::West,
                    Point::new((self.get_width() - 1) as i32, y as i32),
                )];
                self.energized = HashMap::new();
                self.energize()
            })
            .max()
            .unwrap();

        let top_max = (0..self.get_width())
            .map(|x| {
                self.fringe = vec![Light::new(Direction::South, Point::new(x as i32, 0))];
                self.energized = HashMap::new();
                self.energize()
            })
            .max()
            .unwrap();

        let bottom_max = (0..self.get_width())
            .map(|x| {
                self.fringe = vec![Light::new(
                    Direction::North,
                    Point::new(x as i32, (self.get_height() - 1) as i32),
                )];
                self.energized = HashMap::new();
                self.energize()
            })
            .max()
            .unwrap();

        cmp::max(cmp::max(left_max, right_max), cmp::max(top_max, bottom_max))
    }
}

fn main() {
    //let file = include_str!("../input/sample.txt");
    let file = include_str!("../input/input.txt");

    let mut new_map = Map::new(file);
    println!("Q1: {:}", new_map.q1());
    println!("Q2: {:}", new_map.q2());
}

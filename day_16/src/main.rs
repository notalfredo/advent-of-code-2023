#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
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

    fn diag_left(&self, point: Point) -> Point {
        match self.direction {
            Direction::North => Point::new(point.x - 1, point.y),
            Direction::South => Point::new(point.x + 1, point.y),
            Direction::West => Point::new(point.x, point.y - 1),
            Direction::East => Point::new(point.x, point.y + 1),
        }
    }

    fn diag_right(&self, point: Point) -> Point {
        match self.direction {
            Direction::North => Point::new(point.x + 1, point.y),
            Direction::South => Point::new(point.x - 1, point.y),
            Direction::West => Point::new(point.x, point.y + 1),
            Direction::East => Point::new(point.x, point.y - 1),
        }
    }

    fn new_lights(&self, next_tile: Tile, next_loc: Point) -> Option<Vec<Light>> {
        match next_tile {
            Tile::Empty => {
                return Some(vec![Light::new(
                    self.direction,
                    self.keep_same_dir(next_loc),
                )]);
            }
            Tile::Vertical => match self.direction {
                Direction::North | Direction::South => {
                    return Some(vec![Light::new(
                        self.direction,
                        self.keep_same_dir(next_loc),
                    )]);
                }
                Direction::West | Direction::East => {
                    let (north, south) = self.split_light(next_loc);
                    return Some(vec![
                        Light::new(Direction::North, north),
                        Light::new(Direction::South, south),
                    ]);
                }
            },
            Tile::Horizontal => match self.direction {
                Direction::North | Direction::South => {
                    let (north, south) = self.split_light(next_loc);
                    return Some(vec![
                        Light::new(Direction::North, north),
                        Light::new(Direction::South, south),
                    ]);
                }
                Direction::West | Direction::East => {
                    return Some(vec![Light::new(
                        self.direction,
                        self.keep_same_dir(next_loc),
                    )]);
                }
            },
            Tile::MirrorL => match self.direction {
                _ => {
                    return Some(vec![Light::new(self.direction, self.diag_left(next_loc))]);
                }
            },
            Tile::MirrorR => match self.direction {
                _ => {
                    return Some(vec![Light::new(self.direction, self.diag_left(next_loc))]);
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
    energized: Vec<Point>,
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
        let energized: Vec<Point> = Vec::new();
        let fringe: Vec<Light> = vec![Light::new(Direction::East, Point::new(0, 0))];
        Self {
            map,
            energized,
            fringe,
        }
    }

    fn q1(&mut self) {
        for _ in 0..1{
            let curr_light: Light = self.fringe.pop().unwrap();
            let next_step = curr_light.get_next_pos();
            let next_tile = self.map[next_step.y as usize][next_step.x as usize];

            println!("{:?}", curr_light);
            println!("{:?}", next_step);
            println!("{:?}", next_tile);

            let next_lights: Vec<Light> =
                Light::new_lights(&curr_light, next_tile, next_step).unwrap();

            println!("{:?}", next_lights);
        }
    }
}

fn main() {
    let file = include_str!("../input/sample.txt");

    let mut new_map = Map::new(file); 
    new_map.q1();

}

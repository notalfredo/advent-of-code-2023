#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn determinant(&self, point_2: &Point) -> i32 {
        (self.x as i32 * point_2.y as i32) - (self.y as i32 * point_2.x as i32)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Pipe {
    pipe_loc: Point,
    pipe_type: char,
}

impl Pipe {
    fn new(pipe_loc: Point, pipe_type: char) -> Self {
        Self {
            pipe_loc,
            pipe_type,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn check_if_valid_move(dir: Direction, c: char) -> Option<Direction> {
        match dir {
            Direction::North => match c {
                '|' => Some(Direction::North),
                '7' => Some(Direction::West),
                'F' => Some(Direction::East),
                _ => None,
            },
            Direction::South => match c {
                '|' => Some(Direction::South),
                'L' => Some(Direction::East),
                'J' => Some(Direction::West),
                _ => None,
            },
            Direction::East => match c {
                '-' => Some(Direction::East),
                'J' => Some(Direction::North),
                '7' => Some(Direction::South),
                _ => None,
            },
            Direction::West => match c {
                '-' => Some(Direction::West),
                'L' => Some(Direction::North),
                'F' => Some(Direction::South),
                _ => None,
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct MazePointer {
    loc: Point,
    curr_dir: Direction,
}

impl MazePointer {
    fn new(loc: Point, curr_dir: Direction) -> Self {
        Self { loc, curr_dir }
    }

    fn make_move(&mut self, map: &Vec<Vec<char>>) {
        match self.curr_dir {
            Direction::North => {
                self.curr_dir = Direction::check_if_valid_move(
                    Direction::North,
                    map[self.loc.x - 1][self.loc.y],
                )
                .unwrap();
                self.loc.x = self.loc.x - 1;
                self.loc.y = self.loc.y;
            }
            Direction::South => {
                self.curr_dir = Direction::check_if_valid_move(
                    Direction::South,
                    map[self.loc.x + 1][self.loc.y],
                )
                .unwrap();
                self.loc.x = self.loc.x + 1;
                self.loc.y = self.loc.y;
            }
            Direction::West => {
                self.curr_dir = Direction::check_if_valid_move(
                    Direction::West,
                    map[self.loc.x][self.loc.y - 1],
                )
                .unwrap();
                self.loc.x = self.loc.x;
                self.loc.y = self.loc.y - 1;
            }

            Direction::East => {
                self.curr_dir = Direction::check_if_valid_move(
                    Direction::East,
                    map[self.loc.x][self.loc.y + 1],
                )
                .unwrap();
                self.loc.x = self.loc.x;
                self.loc.y = self.loc.y + 1;
            }
        }
    }
}

struct Maze {
    tiles: Vec<Vec<char>>,
    start_pos: Point,
}

impl Maze {
    fn dump(&self) {
        for line in &self.tiles {
            println!("{:?}", line);
        }
        println!("({:}, {:})", self.start_pos.x, self.start_pos.y);
    }

    fn new(file: &str) -> Self {
        let mut pos = (0, 0);
        let tiles = file
            .lines()
            .enumerate()
            .map(|(index, line)| {
                if let Some(loc) = line.chars().position(|c| c == 'S') {
                    pos = (index, loc);
                }
                line.chars().collect::<Vec<char>>()
            })
            .collect::<Vec<_>>();

        Self {
            tiles,
            start_pos: Point::new(pos.0, pos.1),
        }
    }

    fn gen_two_starting_pointers(&self) -> (MazePointer, MazePointer) {
        let mut pointers: Vec<MazePointer> = Vec::new();

        if let Some(new_dir) = Direction::check_if_valid_move(
            Direction::South,
            self.tiles[self.start_pos.x + 1][self.start_pos.y],
        ) {
            pointers.push(MazePointer::new(
                Point::new(self.start_pos.x + 1, self.start_pos.y),
                new_dir,
            ))
        }
        if let Some(new_dir) = Direction::check_if_valid_move(
            Direction::North,
            self.tiles[self.start_pos.x - 1][self.start_pos.y],
        ) {
            pointers.push(MazePointer::new(
                Point::new(self.start_pos.x - 1, self.start_pos.y),
                new_dir,
            ))
        }
        if let Some(new_dir) = Direction::check_if_valid_move(
            Direction::East,
            self.tiles[self.start_pos.x][self.start_pos.y + 1],
        ) {
            pointers.push(MazePointer::new(
                Point::new(self.start_pos.x, self.start_pos.y + 1),
                new_dir,
            ))
        }
        if let Some(new_dir) = Direction::check_if_valid_move(
            Direction::West,
            self.tiles[self.start_pos.x][self.start_pos.y - 1],
        ) {
            pointers.push(MazePointer::new(
                Point::new(self.start_pos.x, self.start_pos.y - 1),
                new_dir,
            ))
        }

        (*pointers.first().unwrap(), *pointers.last().unwrap())
    }

    fn gen_edges(&self) -> (Vec<Pipe>, Vec<Pipe>) {
        let mut pointer_one_visted_points: Vec<Pipe> = Vec::new();
        let mut pointer_two_visted_points: Vec<Pipe> = Vec::new();

        let (mut pointer_one, mut pointer_two): (MazePointer, MazePointer) =
            self.gen_two_starting_pointers();

        pointer_one_visted_points.push(Pipe::new(
            Point::new(self.start_pos.x, self.start_pos.y),
            'S',
        ));

        pointer_one_visted_points.push(Pipe::new(
            pointer_one.loc,
            self.tiles[pointer_one.loc.x][pointer_one.loc.y],
        ));
        pointer_two_visted_points.push(Pipe::new(
            pointer_two.loc,
            self.tiles[pointer_two.loc.x][pointer_two.loc.y],
        ));

        loop {
            pointer_one.make_move(&self.tiles);
            pointer_two.make_move(&self.tiles);

            if pointer_one.loc == pointer_two.loc {
                pointer_two_visted_points.push(Pipe::new(
                    pointer_two.loc,
                    self.tiles[pointer_two.loc.x][pointer_two.loc.y],
                ));
                break;
            }

            pointer_one_visted_points.push(Pipe::new(
                pointer_one.loc,
                self.tiles[pointer_one.loc.x][pointer_one.loc.y],
            ));
            pointer_two_visted_points.push(Pipe::new(
                pointer_two.loc,
                self.tiles[pointer_two.loc.x][pointer_two.loc.y],
            ));
        }

        (pointer_one_visted_points, pointer_two_visted_points)
    }

    fn q1(&self) -> usize {
        let (_, second_edges) = self.gen_edges();
        second_edges.len()
    }

    //https://en.wikipedia.org/wiki/Pick%27s_theorem
    //https://en.wikipedia.org/wiki/Shoelace_formula
    fn q2(&self) -> i32 {
        let (mut first_edges, second_edges) = self.gen_edges();

        let edge_count: i32 = (first_edges.len() + second_edges.len()) as i32;

        let mut second_edges_rev = second_edges.into_iter().rev().collect::<Vec<_>>();
        first_edges.append(&mut second_edges_rev);

        let vertices: Vec<Point> = first_edges
            .into_iter()
            .filter_map(|point| match point.pipe_type {
                'L' | 'J' | '7' | 'F' | 'S' => Some(point.pipe_loc),
                _ => None,
            })
            .collect();

        let mut index_one = 0;
        let mut index_two = 1;
        let mut total: i32 = 0;

        loop {
            total += vertices[index_one].determinant(&vertices[index_two]);

            if index_two + 1 == vertices.len() {
                total += vertices[index_one + 1].determinant(&vertices[0]);
                break;
            } else {
                index_one += 1;
                index_two += 1;
            }
        }

        (total / 2) - (edge_count / 2) + 1
    }
}

fn main() {
    //let file = include_str!("../input/sample_one.txt");
    //let file = include_str!("../input/sample_three.txt");
    //let file = include_str!("../input/sample_four.txt");
    //let file = include_str!("../input/sample_five.txt");
    //let file = include_str!("../input/sample_six.txt");
    let file = include_str!("../input/input.txt");
    let maze = Maze::new(&file);
    println!("Q1: {:}", maze.q1());
    println!("============");
    println!("Q2: {:}", maze.q2());
}

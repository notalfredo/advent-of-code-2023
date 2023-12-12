#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
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

    fn q1(&self) -> u32 {
        let (mut pointer_one, mut pointer_two) = self.gen_two_starting_pointers();
        let mut count: u32 = 1;

        while pointer_one.loc != pointer_two.loc {
            pointer_one.make_move(&self.tiles);
            pointer_two.make_move(&self.tiles);
            count += 1;
        }
        count
    }
}

fn main() {
    let file = include_str!("../input/input.txt");
    let maze = Maze::new(&file);
    println!("{:}", maze.q1());
}

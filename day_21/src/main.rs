use std::collections::HashSet;

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn gen_north(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn gen_south(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn gen_east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn gen_west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    GardenPlots,
    Rocks,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::GardenPlots,
            '#' => Self::Rocks,
            _ => Self::GardenPlots,
        }
    }
}

struct GardenMap {
    map: Vec<Vec<Tile>>,
    rock_loc: HashSet<Point>,
    start_pos: Point,
}

impl GardenMap {
    fn new(file: &str) -> Self {
        let mut start_pos: Point = Point::new(0, 0);
        let mut rock_loc: HashSet<Point> = HashSet::new();

        let map: Vec<Vec<Tile>> = file
            .lines()
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start_pos = Point::new(x, y);
                        }

                        let tile = Tile::from(c);
                        match tile {
                            Tile::Rocks => {
                                rock_loc.insert(Point::new(x, y));
                            }
                            _ => (),
                        }
                        tile
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect();

        Self {
            map,
            rock_loc,
            start_pos,
        }
    }
    fn get_height(&self) -> usize {
        self.map.len()
    }
    fn get_width(&self) -> usize {
        self.map[0].len()
    }
    fn dump(&self) {
        for row in &self.map {
            println!("{:?}", row);
        }
        println!("");
        for rock_loc in &self.rock_loc {
            println!("{:?}", rock_loc);
        }
        println!("");
        println!("Start loc {:?}", self.start_pos);
    }
    fn within_bounds(&self, x: isize, y: isize) -> bool {
        ((x >= 0) && (x <= (self.get_width() as isize)))
            && ((y >= 0) && (y <= (self.get_height() as isize)))
    }

    fn gen_points(&self, current_steps: &Vec<Point>) -> Vec<Point> {
        let mut known_gened_steps: HashSet<Point> = HashSet::new();
        let mut gened_steps: Vec<Point> = Vec::new();

        for step in current_steps {
            //Gen North
            if self.within_bounds(step.x as isize, (step.y as isize) - 1)
                && self.rock_loc.get(&Point::new(step.x, step.y - 1)).is_none()
            {
                let new_step = step.gen_north();

                if known_gened_steps.insert(new_step) {
                    gened_steps.push(new_step);
                }
            }
            //Gen South
            if self.within_bounds(step.x as isize, (step.y as isize) + 1)
                && self.rock_loc.get(&Point::new(step.x, step.y + 1)).is_none()
            {
                let new_step = step.gen_south();

                if known_gened_steps.insert(new_step) {
                    gened_steps.push(new_step);
                }
            }
            //Gen West
            if self.within_bounds((step.x as isize) - 1, step.y as isize)
                && self.rock_loc.get(&Point::new(step.x - 1, step.y)).is_none()
            {
                let new_step = step.gen_west();

                if known_gened_steps.insert(new_step) {
                    gened_steps.push(new_step);
                }
            }
            //Gen East
            if self.within_bounds((step.x as isize) + 1, step.y as isize)
                && self.rock_loc.get(&Point::new(step.x + 1, step.y)).is_none()
            {
                let new_step = step.gen_east();

                if known_gened_steps.insert(new_step) {
                    gened_steps.push(new_step);
                }
            }
        }
        gened_steps
    }
    fn expand(&self, steps: usize) -> usize {
        let mut current_steps: Vec<Point> = vec![self.start_pos.clone()];

        for _ in 0..steps {
            let test = self.gen_points(&current_steps);
            current_steps = test;
        }
        current_steps.len()
    }
}

fn main() {
    //let file = include_str!("../input/sample.txt");
    let file = include_str!("../input/input.txt");

    let garden_map = GardenMap::new(file);
    println!("Q1: {:}", garden_map.expand(64));
}

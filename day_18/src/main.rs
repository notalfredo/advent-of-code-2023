#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Tried from on a unsupported char"),
        }
    }
}

#[derive(Debug)]
struct Step<'a> {
    dir: Direction,
    step_count: u32,
    color: &'a str,
}

impl<'a> Step<'a> {
    fn new(line: &'a str) -> Self {
        let (dir, steps_color) = line.split_once(' ').unwrap();
        let (step_count, color) = steps_color.split_once(' ').unwrap();

        let dir: Direction = Direction::from(dir.chars().next().expect("string is empty"));
        let step_count: u32 = step_count.parse::<u32>().expect("Unable to parse to int");
        let color: &str = &color[1..color.len() - 1];

        Self {
            dir,
            step_count,
            color,
        }
    }
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

struct Map {
    vertices: Vec<(Point, Point)>,
}

impl Map {
    fn new(first: Point, second: Point) -> Self {
        Self {
            vertices: vec![(first, second)],
        }
    }
    fn length(&self) -> usize {
        self.vertices.len()
    }
    fn get_last(&self) -> Point {
        self.vertices[self.length() - 1].1
    }
    fn insert(&mut self, left: Point, right: Point) {
        self.vertices.push((left, right));
    }
    fn dump(&self) {
        for row in &self.vertices {
            println!("{:?}", row);
        }
    }
    fn calc_area(&self) -> i32 {
        let mut sum: i32 = 0;

        for (left, right) in &self.vertices {
            sum += ((left.x * right.y) as i32) - ((left.y * right.x) as i32);
        }

        sum / 2
    }
}

struct DigPlan<'a> {
    plan: Vec<Step<'a>>,
}

impl<'a> DigPlan<'a> {
    fn new(file: &'a str) -> Self {
        let plan: Vec<Step> = file.lines().map(|line| Step::new(line)).collect();

        Self { plan }
    }
    fn plan_size(&self) -> usize {
        self.plan.len()
    }
    fn dump(&self) {
        for step in &self.plan {
            println!("{:?}", step);
        }
    }

    fn calc_edge_pair(&self, prev: Point, step_index: usize, step_count: &mut u32) -> Point {
        *step_count += self.plan[step_index].step_count;
        match self.plan[step_index].dir {
            Direction::Up => {
                return Point::new(prev.x, prev.y - (self.plan[step_index].step_count as i32));
            }
            Direction::Down => {
                return Point::new(prev.x, prev.y + (self.plan[step_index].step_count as i32));
            }
            Direction::Left => {
                return Point::new(prev.x - (self.plan[step_index].step_count as i32), prev.y);
            }
            Direction::Right => {
                return Point::new(prev.x + (self.plan[step_index].step_count as i32), prev.y);
            }
        }
    }

    fn build_map(&self) -> u32 {
        let mut step_count: u32 = 0;

        let mut map = Map::new(
            Point::new(0, 0),
            self.calc_edge_pair(Point::new(0, 0), 0, &mut step_count),
        );

        let mut step_index = 1;

        while step_index != self.plan_size() {
            let last_point: Point = map.get_last();
            map.insert(
                last_point,
                self.calc_edge_pair(last_point, step_index, &mut step_count),
            );
            step_index += 1;
        }

        step_count + ((map.calc_area() as u32) - (step_count / 2) + 1)
    }
}

fn main() {
    //let file = include_str!("../input/sample.txt");
    //let file = include_str!("../input/sample_two.txt");
    let file = include_str!("../input/input.txt");

    let plan = DigPlan::new(file);
    println!("{:}", plan.build_map());
}

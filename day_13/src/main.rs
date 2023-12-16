#[derive(Debug)]
struct Smudge(bool);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

impl Reflection {
    fn check_against_known(&self, rhs: Option<Reflection>) -> bool {
        match rhs {
            Some(_rhs) => {
                if *self == _rhs {
                    return false;
                }
                true
            }
            None => true,
        }
    }
}

impl Reflection {
    fn get_value(&self) -> usize {
        match self {
            Self::Horizontal(val) => val * 100,
            Self::Vertical(val) => *val,
        }
    }
}

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(grid: &str) -> Self {
        let grid = grid
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Self { grid }
    }
    fn dump(&self) {
        self.grid.iter().for_each(|row| println!("{:?}", row));
        println!("");
    }
    fn equal_row(&self, row_index_one: usize, row_index_two: usize) -> bool {
        self.grid[row_index_one] == self.grid[row_index_two]
    }
    fn equal_column(&self, column_index_one: usize, column_index_two: usize) -> bool {
        let left_column: Vec<char> = (0..self.get_rows_count())
            .map(|row_index| self.grid[row_index][column_index_one])
            .collect();
        let right_column: Vec<char> = (0..self.get_rows_count())
            .map(|row_index| self.grid[row_index][column_index_two])
            .collect();
        left_column == right_column
    }

    fn get_rows_count(&self) -> usize {
        self.grid.len()
    }
    fn get_column_count(&self) -> usize {
        self.grid[0].len()
    }

    fn flip_smudge(&mut self, row_index: usize, column_index: usize) {
        match self.grid[row_index][column_index] {
            '.' => self.grid[row_index][column_index] = '#',
            '#' => self.grid[row_index][column_index] = '.',
            _ => panic!("UNKOWN SYMBOL"),
        }
    }
}

fn q1(grid: &Grid, known_reflection: Option<Reflection>) -> Option<Reflection> {
    for row_index in 0..grid.get_rows_count() - 1 {
        let mut first_pointer: usize = row_index;
        let mut second_pointer: usize = row_index + 1;
        let mut found_equal = grid.equal_row(first_pointer, second_pointer);
        let reflection: usize = second_pointer;

        while (first_pointer as isize - 1 >= 0) && (second_pointer + 1 <= grid.get_rows_count() - 1)
        {
            if !grid.equal_row(first_pointer - 1, second_pointer + 1) {
                found_equal = false;
                break;
            }
            first_pointer -= 1;
            second_pointer += 1;
        }

        if found_equal && Reflection::Horizontal(reflection).check_against_known(known_reflection) {
            return Some(Reflection::Horizontal(reflection));
        }
    }

    for column_index in 0..grid.get_column_count() - 1 {
        let mut first_pointer: usize = column_index;
        let mut second_pointer: usize = column_index + 1;
        let mut found_equal = grid.equal_column(first_pointer, second_pointer);
        let reflection: usize = second_pointer;

        while (first_pointer as isize - 1 >= 0)
            && (second_pointer + 1 <= grid.get_column_count() - 1)
        {
            if !grid.equal_column(first_pointer - 1, second_pointer + 1) {
                found_equal = false;
                break;
            }
            first_pointer -= 1;
            second_pointer += 1;
        }

        if found_equal && Reflection::Vertical(reflection).check_against_known(known_reflection) {
            return Some(Reflection::Vertical(reflection));
        }
    }
    None
}

fn q2(grid: &mut Grid) -> Reflection {
    let pre_smudge = q1(&grid, None);

    for row in 0..grid.get_rows_count() {
        for column in 0..grid.get_column_count() {
            grid.flip_smudge(row, column);

            match q1(&grid, pre_smudge) {
                Some(reflection) => {
                    return reflection;
                }
                None => {
                    grid.flip_smudge(row, column);
                    continue;
                }
            }
        }
    }
    pre_smudge.unwrap()
}

fn parse_grid(file: &str) -> Vec<Grid> {
    file.split("\n\n").map(|grid| Grid::new(grid)).collect()
}

fn main() {
    let file = include_str!("../input/input.txt");
    //let file = include_str!("../input/sample.txt");
    //let file = include_str!("../input/sampe_two.txt");
    //let file = include_str!("../input/sample_three.txt");
    //let file = include_str!("../input/sample_four.txt");

    let grids = parse_grid(file);

    println!(
        "{:?}",
        grids
            .iter()
            .map(|grid| q1(grid, None).unwrap().get_value())
            .sum::<usize>()
    );

    println!(
        "{:?}",
        grids
            .into_iter()
            .map(|mut grid| q2(&mut grid).get_value())
            .sum::<usize>()
    );
}

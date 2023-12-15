#[derive(Debug)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
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
    fn in_bounds(&self, top: isize, bottom: usize, len: usize) -> bool {
        (top >= 0) && (bottom <= len - 1)
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

    fn q1(&self) -> Option<Reflection> {
        for row_index in 0..self.get_rows_count() - 1 {
            let mut first_pointer: usize = row_index;
            let mut second_pointer: usize = row_index + 1;
            let mut found_equal = self.equal_row(first_pointer, second_pointer);
            let reflection: usize = second_pointer;

            while (first_pointer as isize - 1 >= 0)
                && (second_pointer + 1 <= self.get_rows_count() - 1)
            {
                if !self.equal_row(first_pointer - 1, second_pointer + 1) {
                    found_equal = false;
                    break;
                }
                first_pointer -= 1;
                second_pointer += 1;
            }

            if found_equal {
                return Some(Reflection::Horizontal(reflection));
            }
        }

        for column_index in 0..self.get_column_count() - 1 {
            let mut first_pointer: usize = column_index;
            let mut second_pointer: usize = column_index + 1;
            let mut found_equal = self.equal_column(first_pointer, second_pointer);
            let reflection: usize = second_pointer;

            while (first_pointer as isize - 1 >= 0)
                && (second_pointer + 1 <= self.get_column_count() - 1)
            {
                if !self.equal_column(first_pointer - 1, second_pointer + 1) {
                    found_equal = false;
                    break;
                }
                first_pointer -= 1;
                second_pointer += 1;
            }

            if found_equal {
                return Some(Reflection::Vertical(reflection));
            }
        }

        None
    }
}

fn parse_grid(file: &str) -> Vec<Grid> {
    file.split("\n\n").map(|grid| Grid::new(grid)).collect()
}

fn main() {
    let file = include_str!("../input/input.txt");
    //let file = include_str!("../input/sample.txt");
    //let file = include_str!("../input/sampe_two.txt");
    //let file = include_str!("../input/sample_three.txt");

    let mut grids = parse_grid(file);

    println!(
        "{:?}",
        grids
            .iter()
            .map(|grid| grid.q1().unwrap().get_value())
            .sum::<usize>()
    );

    //println!("{:}", grids.iter_mut().map(|a| {

    //    12

    //}).sum::<usize>());
}

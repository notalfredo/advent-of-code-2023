fn gen_permutations(total_galaxy: usize) -> Vec<Pair> {
    let mut pairs: Vec<Pair> = Vec::new();

    for i in 1..=total_galaxy {
        for j in i + 1..total_galaxy {
            pairs.push(Pair::new(i, j, 0));
        }
    }

    pairs
}

struct Extended {
    exteneded_rows: Vec<usize>,
    exteneded_columns: Vec<usize>,
}

impl Extended {
    fn new() -> Self {
        Self {
            exteneded_rows: Vec::new(),
            exteneded_columns: Vec::new(),
        }
    }
    fn dump(&self) {
        println!("{:?}", self.exteneded_rows);
        println!("{:?}", self.exteneded_columns);
    }
}

fn expand_map(file: &str) -> Extended {
    let mut rows_expanded: Extended = Extended::new();
    let mut file_vec: Vec<Vec<char>> = Vec::new();

    file.lines().enumerate().for_each(|(row_num, line)| {
        let line_chars = line.chars().collect::<Vec<_>>();
        let line_chars_filtered = line.chars().filter(|c| *c == '#').collect::<Vec<_>>();

        if line_chars_filtered.len() == 0 {
            rows_expanded.exteneded_rows.push(row_num);
        }
        file_vec.push(line_chars);
    });

    let column_count = file_vec[0].len();
    let row_count = file_vec.len();

    for column in 0..column_count {
        let mut found_pound: bool = false;
        for row in 0..row_count {
            if file_vec[row][column] == '#' {
                found_pound = true;
                break;
            }
        }

        if !found_pound {
            rows_expanded.exteneded_columns.push(column);
        }
    }

    rows_expanded
}

#[derive(Debug)]
struct Pair {
    start_number: usize,
    end_number: usize,
    dist: usize,
}

impl Pair {
    fn new(start_number: usize, end_number: usize, dist: usize) -> Self {
        Self {
            start_number,
            end_number,
            dist,
        }
    }
}

#[derive(Debug)]
struct Galaxy {
    row: usize,
    column: usize,
    number: usize,
}

impl Galaxy {
    fn new(row: usize, column: usize, number: usize) -> Self {
        Self {
            row,
            column,
            number,
        }
    }
}

struct Image {
    galaxy_loc: Vec<Galaxy>,
    pairs: Vec<Pair>,
}

impl Image {
    fn dump(&self) {
        println!("========");
        println!("{:?}", self.galaxy_loc);
        println!("========");
        println!("{:?}", self.pairs);
    }

    fn new(file: &str) -> Self {
        let map: Vec<Vec<char>> = file
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let mut galaxy_loc: Vec<Galaxy> = Vec::new();
        let mut count: usize = 1;

        map.iter().enumerate().for_each(|(row_num, row)| {
            row.iter().enumerate().for_each(|(column_num, c)| {
                if *c == '#' {
                    galaxy_loc.push(Galaxy::new(row_num, column_num, count));
                    count += 1;
                }
            });
        });

        let pairs = gen_permutations(count);
        Self { galaxy_loc, pairs }
    }

    fn find_count(&mut self, mult_rate: usize, extended_info: &Extended) -> usize {
        for pair in &mut self.pairs {
            let first_galaxy = self
                .galaxy_loc
                .iter()
                .find(|galaxy| galaxy.number == pair.start_number)
                .unwrap();
            let second_galaxy = self
                .galaxy_loc
                .iter()
                .find(|galaxy| galaxy.number == pair.end_number)
                .unwrap();

            let (mut curr_row, mut curr_column) = (first_galaxy.row, first_galaxy.column);
            let (end_row, end_column) = (second_galaxy.row, second_galaxy.column);
            let mut step_count = 0;

            while (curr_row != end_row) || (curr_column != end_column) {
                if curr_row < end_row {
                    curr_row += 1;

                    if extended_info
                        .exteneded_rows
                        .iter()
                        .find(|ext_row| **ext_row == curr_row)
                        .is_some()
                    {
                        step_count += 1 * mult_rate;
                    } else {
                        step_count += 1;
                    }
                } else if curr_row > end_row {
                    curr_row -= 1;

                    if extended_info
                        .exteneded_rows
                        .iter()
                        .find(|ext_row| **ext_row == curr_row)
                        .is_some()
                    {
                        step_count += 1 * mult_rate;
                    } else {
                        step_count += 1;
                    }
                }
                if curr_column < end_column {
                    curr_column += 1;

                    if extended_info
                        .exteneded_columns
                        .iter()
                        .find(|ext_column| **ext_column == curr_column)
                        .is_some()
                    {
                        step_count += 1 * mult_rate;
                    } else {
                        step_count += 1;
                    }
                } else if curr_column > end_column {
                    curr_column -= 1;

                    if extended_info
                        .exteneded_columns
                        .iter()
                        .find(|ext_column| **ext_column == curr_column)
                        .is_some()
                    {
                        step_count += 1 * mult_rate;
                    } else {
                        step_count += 1;
                    }
                }
            }

            pair.dist = step_count;
        }

        self.pairs.iter().map(|pair| pair.dist).sum::<usize>()
    }
}

fn main() {
    //let file = include_str!("../input/sample_one.txt");
    //let file = include_str!("../input/sample_two.txt");
    let file = include_str!("../input/input.txt");
    let mut data = Image::new(file);
    let exp_map = expand_map(file);

    println!("Q1: {:}", data.find_count(2, &exp_map));
    println!("============");
    println!("Q2: {:}", data.find_count(1000000, &exp_map));
}

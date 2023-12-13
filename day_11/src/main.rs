fn gen_permutations(total_galaxy: usize) -> Vec<Pair> {
    let mut pairs: Vec<Pair> = Vec::new();

    for i in 1..=total_galaxy {
        for j in i+1..total_galaxy {
            pairs.push( Pair::new(i, j, 0)); 
        }
    }

    pairs
}

fn expand_map(file: &str) -> Vec<Vec<char>> {
    let mut rows_expanded: Vec<Vec<char>> = Vec::new();

    file.lines().for_each(|line| {
        let line_chars = line.chars().collect::<Vec<_>>();
        let line_chars_filtered = line.chars().filter(|c| *c == '#').collect::<Vec<_>>();

        if line_chars_filtered.len() == 0 {
            rows_expanded.push( (0..line_chars.len()).into_iter().map(|_| '.').collect::<Vec<char>>());
            rows_expanded.push( (0..line_chars.len()).into_iter().map(|_| '.').collect::<Vec<char>>());
        }
        else {
            rows_expanded.push(line_chars);
        }
    });

    
    let mut columns_expanded: Vec<Vec<char>> = Vec::with_capacity( rows_expanded.len() );
    for _ in 0..rows_expanded.len() {

        columns_expanded.push( Vec::new() );
    }
    
    let column_count = rows_expanded[0].len();
    let row_count = rows_expanded.len();

    for column in 0..column_count {
        let mut found_pound: bool = false;
        for row in 0..row_count {
            if rows_expanded[row][column] == '#' {
                found_pound = true; 
                break;
            }
        }
        
        for row in 0..row_count {
            if found_pound {
                columns_expanded[row].push(rows_expanded[row][column]);
            }
            else {
                columns_expanded[row].push('.');
                columns_expanded[row].push('.');
            }
        }
    }
    
    columns_expanded
}


#[derive(Debug)]
struct Pair {
    start_number: usize,
    end_number: usize,
    dist: usize
}


impl Pair {
    fn new(start_number: usize, end_number: usize, dist: usize) -> Self {
        Self { start_number, end_number, dist}
    }
}


#[derive(Debug)]
struct Galaxy {
    row: usize,
    column: usize,
    number: usize
}

impl Galaxy {
    fn new(row: usize, column: usize, number: usize) -> Self {
        Self { row, column, number }
    }
}

struct Image {
    map: Vec<Vec<char>>,
    galaxy_loc: Vec<Galaxy>,
    pairs: Vec<Pair>
}


impl Image {

    fn dump(&self) {
        for row in &self.map {
            println!("{:?}", row);
        }
        println!("========");
        println!("{:?}", self.galaxy_loc);
        println!("========");
        println!("{:?}", self.pairs);
    }

    fn new(file: &str) -> Self {
        let map = expand_map(file);

        let mut galaxy_loc: Vec<Galaxy> = Vec::new();
        let mut count: usize = 1;

        map.iter().enumerate().for_each(|(row_num, row)|{
            row.iter().enumerate().for_each(|(column_num, c)| {
                if *c == '#' {
                    galaxy_loc.push(Galaxy::new(row_num, column_num, count));
                    count += 1;
                }
            });
        });

        let pairs = gen_permutations(count);
        Self { map, galaxy_loc, pairs }
    }   

    fn q1(&mut self) -> usize {
       
        for pair in &mut self.pairs {
            let first_galaxy = self.galaxy_loc.iter().find(|galaxy| galaxy.number == pair.start_number).unwrap();
            let second_galaxy = self.galaxy_loc.iter().find(|galaxy| galaxy.number == pair.end_number).unwrap();

            let (mut curr_row, mut curr_column) = (first_galaxy.row, first_galaxy.column);
            let (end_row, end_column) = (second_galaxy.row, second_galaxy.column);
            let mut step_count = 0;


            while (curr_row != end_row) || (curr_column != end_column) {
                if curr_row < end_row {
                    curr_row += 1;
                    step_count += 1;
                }
                else if curr_row > end_row {
                    curr_row -= 1;
                    step_count += 1;
                }
                if curr_column < end_column {
                    curr_column += 1;
                    step_count += 1;
                }
                else if curr_column > end_column {
                    curr_column -= 1;
                    step_count += 1;
                }
            }

            pair.dist = step_count;
        }

        self.pairs.iter().map(|pair| pair.dist * 5 ).sum::<usize>() 
    }


}

fn main() {
    let file = include_str!("../input/sampe_one.txt");
    //let file = include_str!("../input/sample_two.txt");
    //let file = include_str!("../input/input.txt");
    let mut data = Image::new(file);


    println!("============");
    println!("============");
    println!("{:}", data.q1());

}

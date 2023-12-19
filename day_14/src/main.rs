mod spin;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Dish {
    dish: Vec<Vec<char>>,
}

impl Dish {
    fn new(file: &str) -> Self {
        let dish: Vec<Vec<char>> = file
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        Self { dish }
    }
    fn dump(&self) {
        for row in &self.dish {
            for column in row {
                print!("{:}", column);
            }
            println!("");
        }
    }

    fn get_height(&self) -> usize {
        self.dish.len()
    }
    fn get_width(&self) -> usize {
        self.dish[0].len()
    }
    fn check_zero(&self, row_index: usize, column_index: usize) -> bool {
        self.dish[row_index][column_index] == 'O'
    }
    fn check_pound(&self, index: usize, column_index: usize) -> bool {
        self.dish[index][column_index] == '#'
    }
    fn check_dot(&self, index: usize, column_index: usize) -> bool {
        self.dish[index][column_index] == '.'
    }

    fn within_bounds(&self, window_top: isize, window_bottom: usize, height: usize) -> bool {
        (window_top >= 0) && (window_bottom <= height)
    }

    fn fill(&mut self, top_index: usize, bottom_index: usize, c: char, column_index: usize) {
        for index in top_index..=bottom_index {
            self.dish[index][column_index] = c;
        }
    }
    fn get_score(&self) -> usize {
        let row_count = self.get_height();
        self.dish
            .iter()
            .enumerate()
            .map(|(num, row)| {
                row.iter()
                    .filter(|c| **c == 'O')
                    .collect::<Vec<&char>>()
                    .len()
                    * (row_count - num)
            })
            .sum::<usize>()
    }
    fn __degub_count_zero(&self) -> usize {
        self.dish
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|c| **c == 'O')
                    .collect::<Vec<&char>>()
                    .len()
            })
            .sum::<usize>()
    }
    fn __degub_count_pound(&self) -> usize {
        self.dish
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|c| **c == '#')
                    .collect::<Vec<&char>>()
                    .len()
            })
            .sum::<usize>()
    }

    fn q1(&mut self) -> usize {
        self.tilt_column_north();
        self.get_score()
    }

    fn detect_cycle(&mut self, cycle_amount: usize) -> (usize, usize) {
        let mut detect_cycle: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

        for i in 0..cycle_amount {
            match detect_cycle.get(&self.dish) {
                Some(start) => return (*start, i - start),
                None => {
                    detect_cycle.insert(self.dish.clone(), i);
                    self.tilt_column_north();
                    self.tilt_row_west();
                    self.tilt_column_south();
                    self.tilt_row_east();
                }
            }
        }
        panic!("Unable to find a cycle");
    }

    fn cycle(&mut self, length: usize) -> usize {
        let (start_index, repeat_index) = self.detect_cycle(length);
        let loop_time = (length - (start_index + repeat_index)) % repeat_index;

        for _ in 0..loop_time {
            self.tilt_column_north();
            self.tilt_row_west();
            self.tilt_column_south();
            self.tilt_row_east();
        }
        self.get_score()
    }
}

fn main() {
    let file = include_str!("../input/input.txt");

    let mut curr_dish = Dish::new(file);

    println!("q1: {:}", curr_dish.q1());

    println!("Q2: {:?}", curr_dish.cycle(1000000000));
}

#[cfg(test)]
mod tests {
    use crate::Dish;
    #[test]
    fn loop_for_repeat() {
        let file = include_str!("../input/input.txt");
        let mut dummy = Dish::new(file);
        let n = 1000000000;
        let (start, repeat) = dummy.detect_cycle(n);

        let mut test_lhs = Dish::new(file);
        let mut test_rhs = Dish::new(file);

        //0 to 142: runs for 143 times
        for _ in 0..start {
            test_lhs.tilt_column_north();
            test_lhs.tilt_row_west();
            test_lhs.tilt_column_south();
            test_lhs.tilt_row_east();
        }

        //0 to 142 + 28 (170): runs for 171 times
        for _ in 0..start + repeat {
            test_rhs.tilt_column_north();
            test_rhs.tilt_row_west();
            test_rhs.tilt_column_south();
            test_rhs.tilt_row_east();
        }

        assert_eq!(test_lhs, test_rhs);
    }
}

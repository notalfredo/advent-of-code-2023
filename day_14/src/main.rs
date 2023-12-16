struct Dish {
    dish: Vec<Vec<char>> 
}

impl Dish {
    fn new(file: &str) -> Self {
        let dish: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect::<Vec<char>>() ).collect();
        Self { dish }
    }
    fn dump(&self) {
        for row in &self.dish{
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
    fn check_zero(&self, index: usize, column_index: usize) -> bool {
        self.dish[index][column_index] == 'O'
    }
    fn check_pound(&self, index: usize, column_index: usize) -> bool {
        self.dish[index][column_index] == '#'
    }

    fn within_bounds(&self, window_top: isize, window_bottom: usize, height: usize) -> bool {
        (window_top >= 0 ) && (window_bottom <= height)
    }

    fn fill(&mut self, top_index: usize, bottom_index: usize, c: char, column_index: usize) {
        //println!("FILL: row {:}, row {:}, column {:}, char {:}", top_index, bottom_index, column_index, c);
        for index in top_index..=bottom_index {
            self.dish[index][column_index] = c;
        }
    }
    fn get_score(&self) -> usize{
        let row_count = self.get_height();
        self.dish.iter().enumerate().map(|(num, row)| {
            row.iter().filter(|c| **c == 'O').collect::<Vec<&char>>().len() * ( row_count - num)
        }).sum::<usize>()
    }
    fn __degub_count_zero(&self) -> usize {
        self.dish.iter().map(|row| {
            row.iter().filter(|c| **c == 'O').collect::<Vec<&char>>().len()
        }).sum::<usize>()
    }


    fn tilt_column_north(&mut self, column_index: usize) {
        let height = self.get_height();
        let mut window_top: usize = 0;
        let mut window_bottom: usize = 0;

        while self.within_bounds(0, window_bottom, height - 1){
            //println!("===============");
            //self.dump();
            //println!("===============");

            if !self.check_zero(window_top, column_index) || !self.check_zero(window_bottom, column_index) {
                //println!("===============");
                //println!("{:}, {:}", window_top, window_bottom);
                //println!("{:}, {:}", self.check_zero(window_top, column_index), self.check_zero(window_bottom, column_index) );
                //println!("skipping {:?}, {:?}", self.dish[window_top][column_index], self.dish[window_top][column_index]);
                //println!("===============");
                window_top += 1;
                window_bottom += 1;
                continue; 
            }

            //Find biggest window size of zeros vertically
            while self.within_bounds(window_top as isize, window_bottom + 1, height - 1)
                    && self.check_zero(window_top, column_index)
                    && self.check_zero(window_bottom + 1, column_index){
                window_bottom += 1;
            }

            //println!("window size {:}, {:}", window_top, window_bottom);

            let mut temp_top = window_top;
            let mut temp_bottom = window_bottom;

            //Try and see how far north you can move it 
            while self.within_bounds(temp_top as isize - 1, temp_bottom, height - 1)
                    && !self.check_pound(temp_top - 1, column_index)
                    && !self.check_zero(temp_top - 1, column_index){
                temp_top -= 1;
                temp_bottom -= 1;
            }

            //println!("new pos {:}, {:}", temp_top, temp_bottom);

            if temp_top != window_bottom && temp_bottom != window_bottom {
                if window_top <= temp_bottom {
                    window_top = temp_bottom + 1;
                }

                self.fill(temp_top, temp_bottom, 'O', column_index);
                self.fill(window_top, window_bottom, '.', column_index);
            }

            window_bottom += 1;
            window_top = window_bottom;
        }

        
    }

    fn q1(&mut self) -> usize{
        println!("\n\nCount zero, {:}", self.__degub_count_zero());

        for column in 0..self.get_width() {
            //println!("{:}", column);
            self.tilt_column_north(column);
        }

        println!("Count zero, {:}\n", self.__degub_count_zero());
        //self.dump();
        self.get_score()
    }

}


fn main() {
    //let file = include_str!("../input/sample.txt");
    let file = include_str!("../input/input.txt");
    //let file = include_str!("../input/sample_two.txt");
    
    let mut curr_dish = Dish::new(file);
    //curr_dish.dump();
    println!("{:}", curr_dish.q1());
}

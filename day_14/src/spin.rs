use crate::Dish;

impl Dish {
    pub fn tilt_column_north_improved(&mut self) {
        for column_index in 0..self.get_width() {
            let mut row = 0;
            let mut found_pound: bool = false;

            while row < self.get_height() - 1 {
                if self.check_zero(row, column_index) || self.check_pound(row, column_index) {
                    row += 1;
                    continue;
                }
                let mut pointer = row + 1;
                while pointer < self.get_height() {
                    if self.check_pound(pointer, column_index) {
                        row = pointer + 1;
                        found_pound = true;
                        break;
                    }
                    if self.check_zero(pointer, column_index) {
                        self.dish[row][column_index] = 'O';
                        self.dish[pointer][column_index] = '.';
                        row += 1;
                    }
                    pointer += 1;
                }
                if found_pound {
                    found_pound = false;
                    continue;
                }
                row += 1;
            }
        }
    }

    pub fn tilt_column_south_improved(&mut self) {
        for column_index in 0..self.get_width() {
            let mut row: isize = (self.get_height() - 1) as isize;
            let mut found_pound: bool = false;

            while row > 0 {
                if self.check_zero(row as usize, column_index)
                    || self.check_pound(row as usize, column_index)
                {
                    row -= 1;
                    continue;
                }
                let mut pointer = row - 1;
                while pointer >= 0 {
                    if self.check_pound(pointer as usize, column_index) {
                        row = pointer - 1;
                        found_pound = true;
                        break;
                    }
                    if self.check_zero(pointer as usize, column_index) {
                        self.dish[row as usize][column_index] = 'O';
                        self.dish[pointer as usize][column_index] = '.';
                        row -= 1;
                    }
                    pointer -= 1;
                }
                if found_pound {
                    found_pound = false;
                    continue;
                }
                row -= 1;
            }
        }
    }

    pub fn tilt_row_west_improved(&mut self) {
        for row_index in 0..self.get_height() {
            //println!("width {:}", self.get_width());
            let mut column: usize = 0;
            let mut found_pound: bool = false;

            while column < self.get_width() - 1 {
                //println!("- column{:}", column);

                if self.check_zero(row_index, column) || self.check_pound(row_index, column) {
                    column += 1;
                    continue;
                }

                let mut pointer = column + 1;
                while pointer < self.get_width() {
                    //println!("- column {:}, pointer {:}", column, pointer);
                    if self.check_pound(row_index, pointer) {
                        column = pointer + 1;
                        found_pound = true;
                        break;
                    }

                    if self.check_zero(row_index, pointer) {
                        //println!("SWAP row {:}, column {:}", row_index, column);
                        self.dish[row_index][column] = 'O';
                        self.dish[row_index][pointer] = '.';
                        column += 1;
                    }
                    pointer += 1;
                }
                if found_pound {
                    found_pound = false;
                    continue;
                }
                column += 1;
            }
        }
    }

    pub fn tilt_row_east_improved(&mut self) {
        for row_index in 0..self.get_height() {
            //println!("width {:}", self.get_width() );
            let mut column: isize = (self.get_width() - 1) as isize;
            let mut found_pound: bool = false;

            while column > 0 {
                //println!("- column{:}", column);

                if self.check_zero(row_index, column as usize)
                    || self.check_pound(row_index, column as usize)
                {
                    column -= 1;
                    continue;
                }

                let mut pointer: isize = (column - 1) as isize;
                while pointer >= 0 {
                    //println!("- column {:}, pointer {:}", column, pointer);
                    if self.check_pound(row_index, pointer as usize) {
                        column = pointer - 1;
                        found_pound = true;
                        break;
                    }

                    if self.check_zero(row_index, pointer as usize) {
                        //println!("SWAP row {:}, column {:}", row_index, column);
                        self.dish[row_index][column as usize] = 'O';
                        self.dish[row_index][pointer as usize] = '.';
                        column -= 1;
                    }
                    pointer -= 1;
                }
                if found_pound {
                    found_pound = false;
                    continue;
                }
                column -= 1;
            }
        }
    }
}

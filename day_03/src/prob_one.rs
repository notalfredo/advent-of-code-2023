#[derive(Debug)]
struct Point {
    row: usize,
    column: usize,
}

#[derive(Debug)]
struct Symbol {
    symbol_point: Point,
    character: char,
}

#[derive(Debug)]
struct Number {
    number_point: Vec<Point>,
    number_char: Vec<char>,
}

struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

fn dump_symbol_vector(symbol_vec: &Vec<Symbol>) {
    for i in symbol_vec {
        println!(
            "row: {:}, column: {:} = {:}",
            i.symbol_point.row, i.symbol_point.column, i.character
        );
    }
}

fn dump_number_vector(number_vec: &Vec<Number>) {
    for number in number_vec {
        println!("{:?}, {:?}", number.number_point, number.number_char);
    }
}

impl Number {
    fn new() -> Number {
        Number {
            number_point: Vec::new(),
            number_char: Vec::new(),
        }
    }

    fn has_adjacent_symbol(&self, vector_of_symbols: &Vec<Symbol>) -> u32 {
        let adjacent_points: Vec<Option<_>> = self.number_point.iter().map(|point| {
              vector_of_symbols.iter().find(|curr_symbol| {
                point.row.abs_diff(curr_symbol.symbol_point.row) <= 1 
                    &&
                point.column.abs_diff(curr_symbol.symbol_point.column) <= 1 
              })
        }).collect();

        let found_adjecent_symbols = adjacent_points.iter().filter(|x| x.is_some() ).collect::<Vec<_>>();


        if found_adjecent_symbols.len() > 0 {
            let mut sum: u32 = 0;
            let mut ten_power: u32 = 1;
            self.number_char.iter().rev().for_each(|c| {
                sum += c.to_digit(10).unwrap() * ten_power;
                ten_power *= 10;
            });
            sum
        }
        else {
            0
        }

    }
}

impl Schematic {
    fn new(numbers: Vec<Number>, symbols: Vec<Symbol>) -> Schematic {
        Schematic { numbers, symbols }
    }
    fn dump(&self) {
        dump_symbol_vector(&self.symbols);
        dump_number_vector(&self.numbers);
    }

    fn get_sum(&mut self) -> u32 {
        let result: &u32 = &self
            .numbers
            .iter()
            .map(|number| number.has_adjacent_symbol(&self.symbols))
            .sum();
        *result
    }
}

fn parse_symbols(lines: &str) -> Vec<Symbol> {
    lines
        .chars()
        .filter(|x| *x != '\n')
        .enumerate()
        .filter(|(_, c)| *c != '.' && !(c.is_digit(10)))
        .map(|(pos, c)| {
            let row_pos = pos / 140;
            Symbol {
                symbol_point: Point {
                    row: row_pos,
                    column: pos - (row_pos * 140),
                },
                character: c,
            }
        })
        .collect()
}

fn parse_numbers(lines: &str) -> Vec<Number> {
    let mut number_vec: Vec<Number> = Vec::new();

    let mut curr_row: usize = 0;
    let mut curr_column: usize = 0;
    let mut found_num = false;

    let mut temp_number: Number = Number::new();

    for c in lines.chars() {
        if c.is_digit(10) {
            found_num = true;
            temp_number.number_point.push(Point {
                row: curr_row,
                column: curr_column,
            });
            temp_number.number_char.push(c);
            curr_column += 1;
        }
        if !c.is_digit(10) {
            if found_num {
                number_vec.push(temp_number);
                temp_number = Number::new();
            }
            found_num = false;
            curr_column += 1;
        }
        if c == '\n' {
            curr_row += 1;
            curr_column = 0;
            found_num = false;
        }
    }
    number_vec
}

pub fn problem_one(lines: &str) -> u32 {
    let mut curr_schematic: Schematic = Schematic::new(parse_numbers(lines), parse_symbols(lines));

    curr_schematic.get_sum()

}

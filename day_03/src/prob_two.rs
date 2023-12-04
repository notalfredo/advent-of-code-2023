#[derive(Debug)]
struct Point {
    row: usize,
    column: usize,
}

#[derive(Debug)]
struct Symbol {
    symbol_point: Point,
    character: char,
    adjacent_numbers_count: u32,
}

#[derive(Debug)]
struct Number {
    number_point: Vec<Point>,
    number_char: Vec<char>,
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
}

fn arr_char_to_num(number_char: &Vec<char>) -> u64 {
    let mut total = 0;
    let mut base = 1;

    for c in number_char.iter().rev() {
        total += c.to_digit(10).unwrap() * base;
        base *= 10;
    }

    total as u64
}

fn calc_gear_adjecent_sum(mut gear_symbol_vec: Vec<Symbol>, number_vec: &Vec<Number>) -> u64 {
    gear_symbol_vec.iter_mut().for_each(|sym| {
        number_vec.iter().for_each(|num| {
            if num
                .number_point
                .iter()
                .find(|point| {
                    sym.symbol_point.row.abs_diff(point.row) <= 1
                        && sym.symbol_point.column.abs_diff(point.column) <= 1
                })
                .is_some()
            {
                sym.adjacent_numbers_count += 1;
            }
        })
    });

    let gear_box_2_adj = gear_symbol_vec
        .into_iter()
        .filter(|sym| sym.adjacent_numbers_count == 2)
        .collect::<Vec<_>>();

    let mut total: u64 = 0;
    let mut point_mult: u64 = 1;
    let _ = gear_box_2_adj.into_iter().for_each(|sym| {
        number_vec.iter().for_each(|num| {
            if num
                .number_point
                .iter()
                .find(|point| {
                    sym.symbol_point.row.abs_diff(point.row) <= 1
                        && sym.symbol_point.column.abs_diff(point.column) <= 1
                })
                .is_some()
            {
                point_mult *= arr_char_to_num(&num.number_char);
            }
        });

        total += point_mult;
        point_mult = 1;
    });
    total
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
                adjacent_numbers_count: 0,
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

pub fn problem_two(lines: &str) -> u64 {
    let symbol_vec = parse_symbols(lines);
    let gear_symbol_vec: Vec<Symbol> = symbol_vec
        .into_iter()
        .filter(|symbol| symbol.character == '*')
        .collect();

    let num_vec = parse_numbers(lines);

    calc_gear_adjecent_sum(gear_symbol_vec, &num_vec)
}

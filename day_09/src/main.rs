struct PascalTree {
    tree: [[i128; 22]; 22],
}

fn factorial(num: i128) -> i128 {
    if num == 0 {
        return 1;
    }
    return num * factorial(num - 1);
}

impl PascalTree {
    fn new() -> PascalTree {
        let mut tree: [[i128; 22]; 22] = [[0; 22]; 22];
        for row in 0..=21 {
            for column in 0..=row {
                //println!("{:}, {:}", row, column);
                if column % 2 != 0 {
                    tree[row][column] = -(factorial(row as i128))
                        / (factorial(column as i128) * factorial((row - column) as i128));
                } else {
                    tree[row][column] = (factorial(row as i128))
                        / (factorial(column as i128) * factorial((row - column) as i128));
                }
            }
        }
        PascalTree { tree }
    }

    fn dump(&self) {
        for row in 0..22 {
            for column in 0..22 {
                if self.tree[row][column] == 0 {
                    break;
                }

                print!("{:}, ", self.tree[row][column]);
            }
            println!("");
        }
    }

    fn gen_coefficent_vector(&self, depth: usize) -> [i128; 22] {
        let mut co_vec: [i128; 22] = [0; 22];
        for column in 0..depth {
            for row in 0..depth {
                co_vec[column] += self.tree[row][column];
            }
        }
        co_vec
    }
}

fn problem_one(line_num: Vec<i128>, co_vec: [i128; 22]) -> i128 {
    line_num
        .iter()
        .zip(co_vec.iter())
        .map(|(left, right)| left * right)
        .sum()
}

fn parse(line: &str) -> Vec<i128> {
    line.split(' ')
        .map(|num| num.parse::<i128>().unwrap())
        .rev()
        .collect::<Vec<i128>>()
}

fn main() {
    //let file = include_str!("../input/sample.txt");
    let file = include_str!("../input/input.txt");

    let new_tree = PascalTree::new();

    println!(
        "{:}",
        file.lines()
            .map(|line| {
                let line_num = parse(line);
                let length = line_num.len();
                problem_one(line_num, new_tree.gen_coefficent_vector(length))
            })
            .sum::<i128>()
    );

    println!("================");

    println!(
        "{:}",
        file.lines()
            .map(|line| {
                let line_num = parse(line).into_iter().rev().collect::<Vec<_>>();
                let length = line_num.len();
                problem_one(line_num, new_tree.gen_coefficent_vector(length))
            })
            .sum::<i128>()
    );
}

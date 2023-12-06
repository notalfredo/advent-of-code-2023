mod problem_one;

fn main() {
    let file = include_str!("../input/problem_one_input.txt");

    println!("{:}", problem_one::problem_one_sol(file));
}

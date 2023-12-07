mod problem_one;
mod problem_two;

fn main() {
    //let file = include_str!("../input/problem_one_input.txt");
    let file = include_str!("../input/problem_one_sample.txt");

    println!("{:}", problem_one::problem_one_sol(file));

    println!("===========");

    let file2 = include_str!("../input/problem_one_input.txt");
    //let file2 = include_str!("../input/problem_one_sample.txt");

    println!("{:}", problem_two::problem_one_sol(file2));
}

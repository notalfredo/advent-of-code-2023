mod problem_one;
mod problem_two;

fn main() {
    let file = include_str!("../input/prob_one_input.txt");

    println!(
        "{:?}",
        file.lines()
            .map(|line| { problem_one::problem_one_sol(line) })
            .sum::<u64>()
    );
    println!("=======");

    let file2 = include_str!("../input/prob_one_input.txt");

    println!("{:?}", problem_two::problem_two_sol(file2));
}

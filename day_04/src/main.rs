mod problem_one;

fn main() {
    let file = include_str!("../input/prob_one_input.txt");

    println!(
        "{:?}",
        file.lines()
            .map(|line| { problem_one::problem_one_sol(line) })
            .sum::<u64>()
    );
}

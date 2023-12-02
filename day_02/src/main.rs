mod prob_one;
mod prob_two;

fn main() {
    let file = include_str!("../input/prob_one_input.txt");
    println!(
        "{:}",
        file.lines().map(|x| prob_one::problem_one(x)).sum::<u32>()
    );

    println!("===================");

    let file = include_str!("../input/prob_two_input.txt");
    println!(
        "{:}",
        file.lines().map(|x| prob_two::problem_two(x)).sum::<u64>()
    );
}

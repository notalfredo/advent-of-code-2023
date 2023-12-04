mod prob_one;
mod prob_two;

fn main() {
    let sample = include_str!("../input/prob_one_input.txt");

    println!("{:}", prob_one::problem_one(sample));

    println!("=============");

    let sample_2 = include_str!("../input/prob_one_input.txt");

    println!("{:}", prob_two::problem_two(sample_2));
}

mod prob_one;

fn main() {
    let sample = include_str!("../input/prob_one_input.txt");

    println!("{:}", prob_one::problem_one(sample));

}

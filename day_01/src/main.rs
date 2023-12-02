mod prob_one; 
mod prob_two;

fn main() {
    let parts = include_str!("../input/prob_one_input.txt").split("\n").collect::<Vec<&str>>();
    println!("{:}", prob_one::problem_one(parts));

    println!("\n\n");


    let parts_2 = include_str!("../input/prob_two_input.txt").split("\n").collect::<Vec<&str>>();
    println!("{:}", prob_two::problem_two(parts_2));
}

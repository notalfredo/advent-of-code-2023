mod prob_one;


fn main() {
    let file = include_str!("../input/prob_one_input.txt");

    println!("{:}", file.lines().map(|x| prob_one::problem_one(x)).sum::<u32>());
}

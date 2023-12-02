mod prob_one; 
fn main() {
    //prob_one::count("./input/input_test.txt");
    

    let parts = include_str!("../input/prob_one_input.txt").split("\n").collect::<Vec<&str>>();
    
    prob_one::problem_one(parts);
}

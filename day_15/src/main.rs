
fn q1(file: &str) -> u32 {
    let split = &file[0..file.len() - 1].split(',').collect::<Vec<&str>>();

    split.into_iter().map(|val|{
        let mut sum: u32 = 0;        
        val.chars().for_each(|c| {
            sum += c as u32;
            sum *= 17;
            sum %= 256;
        });
        sum
    }).sum::<u32>()
}



fn main() {
    //let file = include_str!("../input/sample.txt");
    //let file = include_str!("../input/sample_two.txt");
    let file = include_str!("../input/input.txt");

    println!("Q1: {:}", q1(&file));
}

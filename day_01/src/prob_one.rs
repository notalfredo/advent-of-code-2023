pub fn problem_one(file_vec: Vec<&str>) -> u64 {
    let mut count: u64 = 0;

    for line in file_vec {

        if line.len() == 0 {
            continue;
        }

        let numerical_vector: Vec<_> = line.chars()
                         .collect::<Vec<_>>()
                         .iter()
                         .filter(|x| x.is_numeric())
                         .map(|x| x.to_digit(10).unwrap() as u64)
                         .collect::<Vec<_>>();

        count += numerical_vector[0] * 10 + numerical_vector[numerical_vector.len() - 1];
    }
    count
}

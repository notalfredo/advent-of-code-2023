const NUMBER_MAP: [(&str, i32); 18] = [ 
    ("one",   1),
    ("two",   2),
    ("three", 3),
    ("four",  4),
    ("five",  5),
    ("six",   6),
    ("seven", 7),
    ("eight", 8),
    ("nine",  9),
    ("1",     1),
    ("2",     2),
    ("3",     3),
    ("4",     4),
    ("5",     5),
    ("6",     6),
    ("7",     7),
    ("8",     8),
    ("9",     9),
];

pub fn problem_two(file_vec: Vec<&str>) -> u32 {

    let mut count: u32 = 0;

    for line in file_vec {

        if line.len() == 0 {
            continue;
        }

        let mut result_vec = Vec::<(usize, &str, u32)>::new();
        

        let numerical_vector: String = line.chars().into_iter().collect();
        NUMBER_MAP.iter().for_each(|(x, y)| {
                result_vec.append(
                                    &mut numerical_vector
                                    .match_indices(*x)
                                    .map(|val| (val.0, val.1, *y as u32))
                                    .collect()
                                 );
            }
        );
        result_vec.sort_by(|(a, _, _), (x, _, _)| a.cmp(x) );
        count += result_vec.first().unwrap().2 * 10 + result_vec.last().unwrap().2;

    }
    count
}

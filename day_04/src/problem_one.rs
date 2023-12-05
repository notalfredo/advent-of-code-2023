pub fn problem_one_sol(line: &str) -> u64 {
    let mut split = line.split('|').collect::<Vec<_>>();

    let card_numbers = split
        .pop()
        .unwrap()
        .split(' ')
        .filter_map(|val| {
            if val.parse::<u64>().is_ok() {
                Some(val.parse::<u64>())
            } else {
                None
            }
        })
        .map(|val| val.unwrap())
        .collect::<Vec<u64>>();

    let winning_numbers = split
        .pop()
        .unwrap()
        .split(':')
        .collect::<Vec<_>>()
        .pop()
        .unwrap()
        .split(' ')
        .filter_map(|val| {
            if val.parse::<u64>().is_ok() {
                Some(val.parse::<u64>())
            } else {
                None
            }
        })
        .map(|val| val.unwrap())
        .collect::<Vec<u64>>();
    drop(split);

    let result = card_numbers
        .into_iter()
        .filter(|card_int| {
            winning_numbers
                .iter()
                .find(|winn_int| *winn_int == card_int)
                .is_some()
        })
        .collect::<Vec<_>>();

    if result.len() == 0 {
        0
    } else {
        1 * (2_u64.pow((result.len() - 1).try_into().unwrap()))
    }
}

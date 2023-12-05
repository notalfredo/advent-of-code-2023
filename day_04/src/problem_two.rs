pub fn problem_two_sol(lines: &str) -> usize {
    let win_count_per_line = lines
        .lines()
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();

    let mut cards: Vec<(usize, usize)> = (1..win_count_per_line.len() + 1)
        .map(|index| (index, 1))
        .collect::<Vec<_>>();

    win_count_per_line
        .iter()
        .enumerate()
        .for_each(|(game_num, temp_count)| {
            let correct_num = game_num + 1;
            let mut win_count = *temp_count;

            while win_count > 0 {
                let cards_to_add = cards
                    .iter()
                    .find(|(index, _)| correct_num == *index)
                    .unwrap()
                    .1;

                let card = &mut cards
                    .iter_mut()
                    .find(|(index, _)| correct_num + win_count == *index);

                card.as_mut().unwrap().1 += cards_to_add;

                win_count -= 1;
            }
        });
    cards.iter().map(|(_, total)| total).sum::<usize>()
}

fn parse_line(line: &str) -> usize {
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

    card_numbers
        .into_iter()
        .filter(|card_int| {
            winning_numbers
                .iter()
                .find(|winn_int| *winn_int == card_int)
                .is_some()
        })
        .collect::<Vec<_>>()
        .len()
}

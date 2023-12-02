const INIT_GAME: [(&str, u32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

pub fn problem_one(line: &str) -> u32 {
    let mut bag = line
        .split(|c: char| c == ':' || c == ';' || c == ',')
        .map(|cell| {
            let mut split = cell.trim().split(|b: char| b == ' ');
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect::<Vec<_>>();
    let val = bag.first().unwrap().1.parse::<u32>().unwrap();
    bag.remove(0);
    let bag_uncovered = bag
        .iter()
        .map(|tuple| {
            let temp = INIT_GAME
                .iter()
                .find(|cell| cell.0 == tuple.1 && cell.1 >= (tuple.0.parse::<u32>().unwrap()));
            temp
        })
        .filter(|x| x.is_none())
        .collect::<Vec<_>>();

    if bag_uncovered.len() != 0 {
        return 0;
    } else {
        return val;
    }
}

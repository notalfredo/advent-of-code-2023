const INIT_GAME: [(&str, u32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

pub fn problem_two(line: &str) -> u64 {
    let bag: Vec<(&str, &str)> = line
        .split(|c: char| c == ':' || c == ';' || c == ',')
        .map(|cell| {
            let mut split = cell.trim().split(|b: char| b == ' ');
            (split.next().unwrap(), split.next().unwrap())
        })
        .skip(1)
        .collect::<Vec<_>>();

    let (mut red_bag, remaning_color): (Vec<(u64, &str)>, Vec<(u64, &str)>) = bag
        .into_iter()
        .map(|(val, color)| (val.parse::<u64>().unwrap(), color))
        .partition(|(val, color)| *color == "red");

    let (mut green_bag, mut blue_bag): (Vec<(u64, &str)>, Vec<(u64, &str)>) = remaning_color
        .into_iter()
        .partition(|(val, color)| *color == "green");

    red_bag.sort_by(|a, b| a.partial_cmp(b).unwrap());
    green_bag.sort_by(|a, b| a.partial_cmp(b).unwrap());
    blue_bag.sort_by(|a, b| a.partial_cmp(b).unwrap());

    red_bag.last().unwrap().0 * green_bag.last().unwrap().0 * blue_bag.last().unwrap().0

    //println!("{:?}", red_bag);
    //println!("{:?}", green_bag);
    //println!("{:?}", blue_bag);
    //println!("{:?}", red_bag.last().unwrap().0 * green_bag.last().unwrap().0 * blue_bag.last().unwrap().0 );
    //println!("=============");

    //bag.remove(0);
    //let bag_uncovered = bag
    //               .iter()
    //               .map(|tuple| {
    //                     let temp = INIT_GAME.iter().find(|cell| {
    //                         cell.0 == tuple.1 && cell.1 >= (tuple.0.parse::<u32>().unwrap())
    //                     });
    //                     temp
    //               })
    //               .filter(|x| x.is_none())
    //               .collect::<Vec<_>>();

    //if bag_uncovered.len() != 0 {
    //    return 0;
    //}
    //else {
    //    return val;
    //}
}

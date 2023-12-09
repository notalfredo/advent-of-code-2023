use std::collections::HashMap;

enum Direction {
    Left,
    Right
}

impl TryFrom<char> for Direction {
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
        }

        Err("COULD NOT CONVERT CHAR TO DIRECTION")
    }
}


type Node = [u8; 3];
const DEST: [u8; 3] = [90, 90, 90];

struct Map {
    directions: Vec<char>,
    start_loc: Node,
    path: HashMap<Node, (Node, Node)>,
}

impl Map {
    fn dump(&self) {
        println!("{:?}", self.directions);
        println!("{:?}", self.start_loc);
        for (key, val) in self.path.iter() {
            println!("key: {:?} val: {:?}", key, val);
        }
    }

    fn new(mut file: Vec<&str>) -> Self {
        let mut path: HashMap<Node, (Node, Node)> = HashMap::new();

        let directions = file.remove(0).chars().collect::<Vec<_>>();
        let mut start_loc: Node = [0, 0, 0];
        let mut inserted_first: bool = false;

        file.iter().for_each(|line| {
            let (loc, dest) = match line.split_once('=') {
                Some(line) => (line.0.trim(), line.1.trim()),
                None => panic!("Split on '-' unsuccesfull"),
            };

            let (dest_left, dest_right) = match dest.split_once(',') {
                Some(dests) => (&dests.0[1..4], &dests.1[1..4]),
                None => panic!("Split on ',' unsuccesfull"),
            };

            let cur_loc: Node = loc
                .chars()
                .map(|c| c as u8)
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
            let dest_left: Node = dest_left
                .chars()
                .map(|c| c as u8)
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
            let dest_right: Node = dest_right
                .chars()
                .map(|c| c as u8)
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();

            path.insert(cur_loc, (dest_left, dest_right));

            if !inserted_first {
                inserted_first = true;
                start_loc = cur_loc;
            }

        });

        Map { directions, start_loc, path }
    }

    fn follow_instructions(&self) {
    }
}


fn main() {
    let mut file = include_str!("../input/input_sample.txt")
        .lines()
        .filter(|line| line.len() != 0)
        .collect::<Vec<&str>>();

    let cur_map = Map::new(file);
    cur_map.dump();

}

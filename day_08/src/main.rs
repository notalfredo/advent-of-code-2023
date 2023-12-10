use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Left,
    Right
}



impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Could not convert")
        }
    }
}


type Node = [char; 3];
const START: Node = ['A', 'A', 'A'];
const DEST:  Node = ['Z', 'Z', 'Z'];


fn gcd(a: u64, b:u64) -> u64 {
    let mut new_a = a;
    let mut new_b = b;
    while new_b != 0 {
        let temp = new_b;
        new_b = new_a % new_b;
        new_a = temp;
    }
    new_a
}

fn lcm(a: u64, b:u64) -> u64 {
    let num = a * b;
    let den = gcd(a, b);
    num / den
}


struct Map {
    directions: Vec<Direction>,
    path: HashMap<Node, (Node, Node)>,
}

impl Map {
    fn dump(&self) {
        println!("{:?}", self.directions);
        for (key, val) in self.path.iter() {
            println!("key: {:?} val: {:?}", key, val);
        }
    }

    fn new(mut file: Vec<&str>) -> Self {
        let mut path: HashMap<Node, (Node, Node)> = HashMap::new();

        let directions = file.remove(0).chars().map(|c| Direction::from(c) ).collect::<Vec<_>>();

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
                .collect::<Vec<char>>()
                .try_into()
                .unwrap();
            let dest_left: Node = dest_left
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap();
            let dest_right: Node = dest_right
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap();

            path.insert(cur_loc, (dest_left, dest_right));


        });

        Map { directions, path }
    }

    fn q1(&self, start_pos: Node, func: fn(Node) -> bool ) -> u32{
        let mut current_pos = start_pos;
        let mut steps = 0;

        loop {
            for dir in &self.directions {
                let mut dest = *self.path.get(&current_pos).unwrap();
                match *dir {
                    Direction::Left => {
                        current_pos = dest.0;
                        dest = *self.path.get(&current_pos).unwrap();
                    }
                    Direction::Right => {
                        current_pos = dest.1;
                        dest = *self.path.get(&current_pos).unwrap();
                    }
                }
                steps += 1;
                if func(current_pos) {
                    return steps;
                }
            }
        }
    }

    fn q2(&self) -> u64 {
        let mut current_pos_arr = self.path.iter().filter_map(|(key, _)|{
            if key[2] == 'A' {
                Some(key)
            }
            else{
                None
            }
        }).collect::<Vec<_>>();

        let mut all_pos: Vec<u64> = Vec::new();


        while current_pos_arr.len() > 0 {
            let curr = current_pos_arr.pop().unwrap();
            all_pos.push(self.q1(*curr, |x: Node| x[2] == 'Z') as u64);
        }
        


        let mut lcd_all_pos = lcm(all_pos.pop().unwrap(), all_pos.pop().unwrap());

        while all_pos.len() > 0 {
            lcd_all_pos = lcm(lcd_all_pos, all_pos.pop().unwrap()); 
        }

        lcd_all_pos
    }
}


fn main() {
    //let file = include_str!("../input/input_sample_three.txt")
    //let mut file = include_str!("../input/input_sample_two.txt")
    //let mut file = include_str!("../input/input_sample.txt")
    let mut file = include_str!("../input/input.txt")
        .lines()
        .filter(|line| line.len() != 0)
        .collect::<Vec<&str>>();

    let cur_map = Map::new(file);
    println!("{:}", cur_map.q1(START, |x: Node| x == DEST ));
    println!("=========");
    println!("{:}", cur_map.q2());

}

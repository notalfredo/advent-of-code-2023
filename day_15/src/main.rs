use std::{collections::HashMap};

fn hash(label: &str) -> u32 {
    let mut sum: u32 = 0;
    label.chars().for_each(|c| {
        sum += c as u32;
        sum *= 17;
        sum %= 256;
    });
    sum
}


#[derive(Debug)]
struct LightBox {
    label: String,
    focal_length: u8
}

impl LightBox {
    fn new(label: String, focal_length: u8) -> Self {
        Self { label, focal_length }
    }
}


fn q1(file: &str) -> u32 {
    let split = &file[0..file.len() - 1].split(',').collect::<Vec<&str>>();
    split.into_iter().map(|val| hash(val)).sum::<u32>()
}


fn new_hashmap() -> HashMap<u8, Vec<LightBox>> {
    let mut boxes: HashMap<u8, Vec<LightBox>> = HashMap::new();
    (0..=255).for_each(|val| {boxes.insert(val, Vec::new());});
    boxes
}

fn q2(file: &str) {
    let mut boxes: HashMap<u8, Vec<LightBox>> = new_hashmap();
    let file = &file[0..file.len() - 1].split(',').collect::<Vec<&str>>();


    for slot in file.iter() {
        if slot.contains('-') {
            let label: &str = &slot[0..slot.len() - 1];
            let box_num = hash(label) as u8;
            match boxes.get_mut(&box_num) {
                Some(vec_box) => {
                    if vec_box.len() != 0 {
                        match vec_box.iter().position(|cur_box| cur_box.label == label ) {
                            Some(pos) => {
                                vec_box.remove(pos);
                            },
                            None => (),
                        }

                    }

                }
                None => {
                    panic!("TRIED TO INSERT A NEW BOX");
                }
,
            }
        }
        else if slot.contains('=') {
            println!("slot: {:}", slot);
            let (label, curr_focal_length) = dbg!(slot[0..slot.len()].split_once('=').unwrap());
            let curr_focal_length = curr_focal_length.parse::<u8>().unwrap();
            let box_num = hash(label) as u8;

            println!("BOX NUM {:}", box_num);

            match boxes.get_mut(&box_num) {
                Some(vec_box) => {
                    match vec_box.iter_mut().find(|cur_box| cur_box.label == label ) {
                        Some(found_box) => {
                            println!("FOUND BOX");
                            found_box.focal_length = curr_focal_length;
                        },
                        None => {
                            println!("label: {:}, focal_length {:}", label, curr_focal_length);
                            vec_box.push(LightBox::new(label.to_string(), curr_focal_length));
                        },
                    }
                }
                None => {
                    panic!("TRIED TO INSERT A NEW BOX");
                },
            }
        }
        println!("CURR BOXES{:?}", boxes);
    }
}


fn main() {
    //let file = include_str!("../input/input.txt");
    let file = include_str!("../input/sample.txt");
    //let file = include_str!("../input/sample_two.txt");
    //let file = include_str!("../input/sample_three.txt");

    println!("Q1: {:}", q1(&file));

    q2(&file)
}

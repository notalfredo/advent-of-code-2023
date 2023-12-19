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


fn q2(file: &str) {
    let mut boxes: HashMap<u8, Vec<LightBox>> = HashMap::new();
    let file = &file[0..file.len() - 1].split(',').collect::<Vec<&str>>();


    for (_, slot) in file.iter().enumerate() {

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
                None => (),
            }
        }
        else if slot.contains('=') {
            println!("slot: {:}", slot);
            let (label, curr_focal_length) = dbg!(slot[0..slot.len() - 1].split_once('=').unwrap());
            let curr_focal_length = curr_focal_length.parse::<u8>().unwrap();
            let box_num = hash(label) as u8;

            match boxes.get_mut(&box_num) {
                Some(vec_box) => {
                    if vec_box.len() != 0 {
                        match vec_box.iter_mut().find(|cur_box| cur_box.label == label ) {
                            Some(found_box) => {
                                found_box.focal_length = curr_focal_length;
                            },
                            None => {
                                vec_box.push(LightBox::new(label.to_string(), curr_focal_length));
                            },
                        }

                    }

                }
                None => {boxes.insert(box_num, Vec::new());},
            }
        }
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

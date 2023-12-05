#[derive(Debug)]
struct Map {
    rows: Vec<(u32, u32, u32)>
}

impl Map {
    fn new() -> Map {
        Map { rows: Vec::new() }
    }
    fn get_len(&self) -> usize {
        self.rows.len() 
    }
}

struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Map>
}

impl Almanac {
    fn new(seeds: Vec<u32>, maps: Vec<Map>) -> Almanac {
        Almanac {
            seeds,
            maps
        }
    }
    fn dump(&self) {
       println!("{:?}", self.seeds); 
       println!("{:?}", self.maps); 
    }
}

fn parse(file: &str) -> Almanac {
    let mut seeds: Vec<u32> = Vec::new();
    let mut maps: Vec<Map> = Vec::new();
    let mut map: Map = Map::new();

    let mut parse_map_num: bool = false;

    for line in file.lines() {
        if line.len() == 0 {
            parse_map_num = false;
        }
        if line.split_once(": ").is_some(){
            let seeds_str = line.split_once(": ").unwrap().1;
            let seed_vec: Vec<_> = seeds_str.split(' ').collect();
            seed_vec.into_iter().for_each(|num| seeds.push(num.parse::<u32>().unwrap()));
        }
        else if line.contains(":"){
            if map.get_len() > 0 {
                maps.push(map);
            }
            map = Map::new();
            parse_map_num = true;
        }
        else if parse_map_num {
            let seed_vec: Vec<_> = line.split(' ').collect();
            let num: Vec<_> = seed_vec.into_iter().map(|num| num.parse::<u32>().unwrap()).collect();
            map.rows.push((num[0], num[1], num[2]));
        }
    }
    maps.push(map); 
   
    Almanac::new(seeds, maps)
}


pub fn problem_one_sol(file: &str) {
    let almanac = parse(file);
    almanac.dump();
}

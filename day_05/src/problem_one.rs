#[derive(Debug)]
struct Map {
    rows: Vec<(u64, u64, u64)>,
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
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn new(seeds: Vec<u64>, maps: Vec<Map>) -> Almanac {
        Almanac { seeds, maps }
    }

    fn map_seed_to_location(&self) -> Vec<u64> {
        let mut location_vec: Vec<u64> = Vec::new();
        self.seeds.iter().for_each(|seed| {
            let mut next_map_val = *seed;

            for map in &self.maps {
                for (dest_range_start, source_range_start, range_len) in &map.rows {
                    if (source_range_start..&(source_range_start + range_len))
                        .contains(&&next_map_val)
                    {
                        next_map_val = (next_map_val - source_range_start) + dest_range_start;
                        break;
                    }
                }
            }
            location_vec.push(next_map_val);
        });
        location_vec
    }

    fn dump(&self) {
        println!("{:?}", self.seeds);
        println!("{:?}", self.maps);
    }
}

fn parse(file: &str) -> Almanac {
    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Vec<Map> = Vec::new();
    let mut map: Map = Map::new();

    let mut parse_map_num: bool = false;

    for line in file.lines() {
        if line.len() == 0 {
            parse_map_num = false;
        }
        if line.split_once(": ").is_some() {
            let seeds_str = line.split_once(": ").unwrap().1;
            let seed_vec: Vec<_> = seeds_str.split(' ').collect();
            seed_vec
                .into_iter()
                .for_each(|num| seeds.push(num.parse::<u64>().unwrap()));
        } else if line.contains(":") {
            if map.get_len() > 0 {
                maps.push(map);
            }
            map = Map::new();
            parse_map_num = true;
        } else if parse_map_num {
            let seed_vec: Vec<_> = line.split(' ').collect();
            let num: Vec<_> = seed_vec
                .into_iter()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();
            map.rows.push((num[0], num[1], num[2]));
        }
    }
    maps.push(map);

    Almanac::new(seeds, maps)
}

pub fn problem_one_sol(file: &str) -> u64 {
    let almanac = parse(file);

    *almanac.map_seed_to_location().iter().min().unwrap()
}

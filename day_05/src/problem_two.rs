use std::u64::MIN;

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

    fn max_value(&self) -> u64 {
        let mut maps_max: Vec<u64> = Vec::new();
        self.maps.iter().for_each(|map| {
            maps_max.push(
                *map.rows
                    .iter()
                    .map(|(_, source_range_start, range_len)| source_range_start + range_len)
                    .collect::<Vec<_>>()
                    .iter()
                    .max()
                    .unwrap(),
            )
        });
        *maps_max.iter().max().unwrap()
    }

    fn check_over_lapping_ranges(&self, vec_to_merge: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
        let mut ranges: Vec<(u64, u64)> = Vec::new();

        for index in vec_to_merge {
            let mut start_range = index.0;
            let mut end_range = start_range + index.0;
            if ranges.len() == 0 {
                ranges.push((start_range, end_range));
            } else {
                let mut temp_ranges: Vec<(u64, u64)> = Vec::new();
                for range in &mut ranges {
                    //    *-----cur-----*
                    //          *-----prev----*
                    if start_range < range.0 && (end_range < range.1 && end_range > range.0) {
                        end_range = range.1;
                    }
                    //               *-----cur-----*
                    //          *-----prev----*
                    else if (start_range > range.0 && start_range < range.1)
                        && end_range > range.1
                    {
                        start_range = range.0;
                    }
                    //      *---------cur---------*
                    //          *-----prev----*
                    else if start_range < range.0 && end_range > range.1 {
                    }
                    //no merge happen
                    else {
                        temp_ranges.push((range.0, range.1));
                    }
                }
                temp_ranges.push((start_range, end_range));
                ranges = temp_ranges;
            }
        }
        ranges
    }

    fn map_seed_to_location(&self) -> u64 {
        let mut seed_range: Vec<(u64, u64)> = Vec::new();

        for index in (0..self.seeds.len() - 1).step_by(2) {
            seed_range.push((self.seeds[index], self.seeds[index] + self.seeds[index + 1]))
        }

        self.maps.iter().for_each(|map| {
            let temp_rows = map.rows.clone();
            let mut new_ranges: Vec<(u64, u64)> = Vec::new();

            while seed_range.len() > 0 {
                let (seed_start, seed_end): (u64, u64) = seed_range.pop().unwrap();
                let mut chunked_down = false;

                for (start_dest, start_range, range) in &temp_rows {
                    let new_start = std::cmp::max(seed_start, *start_range);
                    let new_end = std::cmp::min(seed_end, *start_range + range);

                    let end_dest = start_dest + range;
                    let end_range = start_range + range;

                    // FOUND MAPPING
                    if new_start < new_end {
                        chunked_down = true;

                        new_ranges.push((
                            start_dest + (new_start - start_range),
                            end_dest - (end_range - new_end),
                        ));

                        //checking for trailing chunk
                        if seed_start < new_start {
                            seed_range.push((seed_start, new_start));
                        } else if new_end < seed_end {
                            seed_range.push((new_end, seed_end));
                        }
                        break;
                    }
                }
                if !chunked_down {
                    new_ranges.push((seed_start, seed_end));
                }
            }
            seed_range = new_ranges;
        });
        *seed_range
            .iter()
            .map(|seed_range| seed_range.0)
            .collect::<Vec<_>>()
            .iter()
            .min()
            .unwrap()
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
    almanac.map_seed_to_location()
}

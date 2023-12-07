struct BoatRace {
    races: Vec<(u64, u64)>,
}

impl BoatRace {
    fn new(file: &str) -> BoatRace {
        let (time, distance) = file.split_once("\n").unwrap();
        let time = time
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split(" ")
            .filter_map(|str_num| {
                if str_num.parse::<u64>().is_ok() {
                    Some(str_num.parse::<u64>())
                } else {
                    None
                }
            })
            .map(|result| result.unwrap())
            .collect::<Vec<_>>();

        let distance = distance
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split(" ")
            .filter_map(|str_num| {
                if str_num.parse::<u64>().is_ok() {
                    Some(str_num.parse::<u64>())
                } else {
                    None
                }
            })
            .map(|result| result.unwrap())
            .collect::<Vec<_>>();

        let races = time
            .into_iter()
            .zip(distance.into_iter())
            .collect::<Vec<_>>()
            .iter()
            .map(|(t, d)| (*t, *d))
            .collect::<Vec<_>>();
        BoatRace { races }
    }

    fn dump(&self) {
        println!("{:?}", self.races);
    }

    fn merge_numbers(&mut self) {
        let times = self
            .races
            .iter()
            .map(|(time, _)| time)
            .fold("".to_string(), |acc, x| acc + &x.to_string())
            .parse::<u64>()
            .unwrap();
        let dist = self
            .races
            .iter()
            .map(|(_, dist)| dist)
            .fold("".to_string(), |acc, x| acc + &x.to_string())
            .parse::<u64>()
            .unwrap();
        self.races = vec![(times, dist)]
    }

    fn solve(&self) -> u64 {
        let mut ranges: Vec<(u64, u64)> = Vec::new();

        for (time_total, end_distance) in &self.races {
            let mut start_holding_time_total = 0;
            let mut end_holding_time_total = 0;

            //Our rate of change is how long we held for
            for rate_of_change in 1..*time_total {
                let time_total_left = *time_total - rate_of_change;
                let distance_traveled = rate_of_change * time_total_left;
                if distance_traveled > *end_distance {
                    start_holding_time_total = rate_of_change;
                    break;
                }
            }
            for rate_of_change in (start_holding_time_total..*time_total).rev() {
                let time_total_left = *time_total - rate_of_change;
                let distance_traveled = rate_of_change * time_total_left;
                if distance_traveled > *end_distance {
                    end_holding_time_total = rate_of_change;
                    break;
                }
            }
            ranges.push((start_holding_time_total, end_holding_time_total));
        }

        let mut mult_total = 1;
        ranges
            .into_iter()
            .map(|(start, end)| (end + 1) - start)
            .for_each(|total| {
                mult_total *= total;
            });
        mult_total
    }
}

fn main() {
    //let file = include_str!("../input/problem_one_sample.txt");
    let file = include_str!("../input/problem_one_input.txt");

    let mut boat_race = BoatRace::new(file);
    println!("{:}", boat_race.solve());
    println!("==================================");
    boat_race.merge_numbers();
    println!("{:}", boat_race.solve());
}

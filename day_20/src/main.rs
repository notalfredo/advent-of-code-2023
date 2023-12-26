use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Copy, Clone)]
enum Status {
    On,
    Off,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum ModuleType {
    FlipFlop,
    Conjuction,
    Broadcast,
}

impl From<&str> for ModuleType {
    fn from(mod_name: &str) -> Self {
        if mod_name == "broadcaster" {
            return ModuleType::Broadcast;
        }
        let symbol = &mod_name[0..1];
        match symbol {
            "%" => {
                return ModuleType::FlipFlop;
            }
            "&" => {
                return ModuleType::Conjuction;
            }
            _ => panic!("Unkown symbol |{:}|, |{:}|", symbol, mod_name),
        }
    }
}

#[derive(Debug, Clone)]
struct Action {
    origin: String,
    pulse: Pulse,
    dest: String,
}

impl Action {
    fn new(origin: String, pulse: Pulse, dest: String) -> Self {
        Self {
            origin,
            pulse,
            dest,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct FlipFlop<'a> {
    name: &'a str,
    current_status: Status,
}

impl<'a> FlipFlop<'a> {
    fn new(name: &'a str, current_status: Status) -> Self {
        Self {
            name,
            current_status,
        }
    }
    fn flip(&mut self) {
        match self.current_status {
            Status::On => self.current_status = Status::Off,
            Status::Off => self.current_status = Status::On,
        }
    }
    fn get_current_status(&self) -> Status {
        self.current_status
    }
}

#[derive(Debug, Clone)]
struct Conjuction<'a> {
    name: &'a str,
    received_history: Vec<(&'a str, Pulse)>,
}

impl<'a> Conjuction<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            received_history: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Module<'a> {
    mod_type: ModuleType,
    flip_info: Option<FlipFlop<'a>>,
    conj_info: Option<Conjuction<'a>>,
    connections: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn new_broadcast(connections: Vec<&'a str>) -> Self {
        Self {
            mod_type: ModuleType::Broadcast,
            flip_info: None,
            conj_info: None,
            connections,
        }
    }

    fn new(lhs: &'a str, rhs: &'a str) -> Self {
        let mod_type = ModuleType::from(lhs);
        let connections: Vec<&str> = rhs.split(',').map(|name| &name[1..]).collect();

        match mod_type {
            ModuleType::FlipFlop => {
                return Self {
                    mod_type,
                    flip_info: Some(FlipFlop::new(&lhs[1..], Status::Off)),
                    conj_info: None,
                    connections,
                }
            }
            ModuleType::Conjuction => {
                return Self {
                    mod_type,
                    flip_info: None,
                    conj_info: Some(Conjuction::new(&lhs[1..])),
                    connections,
                }
            }
            ModuleType::Broadcast => {
                return Self {
                    mod_type,
                    flip_info: None,
                    conj_info: None,
                    connections,
                }
            }
        }
    }

    fn update_and_send_signal(&mut self, action: Action) -> Option<Vec<Action>> {
        match self.mod_type {
            ModuleType::FlipFlop => match action.pulse {
                Pulse::High => {
                    return None;
                }
                Pulse::Low => {
                    self.flip_info.as_mut().unwrap().flip();
                    return Some(self.gen_actions());
                }
            },
            ModuleType::Conjuction => {
                match self
                    .conj_info
                    .as_mut()
                    .unwrap()
                    .received_history
                    .iter_mut()
                    .find(|(name, _last_pulse)| *name == action.origin)
                {
                    Some((_, last_pulse)) => {
                        *last_pulse = action.pulse;
                        return Some(self.gen_actions());
                    }
                    None => {
                        panic!("Unable to find action origin in conjuction history");
                    }
                }
            }
            _ => panic!("How did you not find a name?"),
        }
    }

    fn gen_actions(&self) -> Vec<Action> {
        match self.mod_type {
            ModuleType::Broadcast => {
                return self
                    .connections
                    .iter()
                    .map(|connection| {
                        Action::new("broadcast".to_string(), Pulse::Low, connection.to_string())
                    })
                    .collect::<Vec<Action>>();
            }
            ModuleType::FlipFlop => match self.flip_info.unwrap().get_current_status() {
                Status::On => {
                    return self
                        .connections
                        .iter()
                        .map(|connection| {
                            Action::new(
                                self.flip_info.unwrap().name.to_string(),
                                Pulse::High,
                                connection.to_string(),
                            )
                        })
                        .clone()
                        .collect();
                }
                Status::Off => {
                    return self
                        .connections
                        .iter()
                        .map(|connection| {
                            Action::new(
                                self.flip_info.unwrap().name.to_string(),
                                Pulse::Low,
                                connection.to_string(),
                            )
                        })
                        .clone()
                        .collect();
                }
            },
            ModuleType::Conjuction => {
                let temp = &self.conj_info.as_ref().unwrap().received_history;
                //println!("{:?}", temp);
                match temp
                    .iter()
                    .find(|(_, hist_pulse)| *hist_pulse == Pulse::Low)
                {
                    Some(_) => {
                        return self
                            .connections
                            .iter()
                            .map(|connection| {
                                Action::new(
                                    self.conj_info.as_ref().unwrap().name.to_string(),
                                    Pulse::High,
                                    connection.to_string(),
                                )
                            })
                            .collect();
                    }
                    None => {
                        return self
                            .connections
                            .iter()
                            .map(|connection| {
                                Action::new(
                                    self.conj_info.as_ref().unwrap().name.to_string(),
                                    Pulse::Low,
                                    connection.to_string(),
                                )
                            })
                            .collect();
                    }
                }
            }
        }
    }
    fn init_conj_info(&mut self, info: &Vec<&'a str>) {
        self.conj_info.as_mut().unwrap().received_history =
            info.iter().map(|name| (*name, Pulse::Low)).collect();
    }
}

#[derive(Debug)]
struct ModuleConfiguration<'a> {
    broadcast_connections: Vec<&'a str>,
    modules: Vec<Module<'a>>,
    low_pulse_count: u64,
    high_pulse_count: u64,
}

impl<'a> ModuleConfiguration<'a> {
    fn new(file: &'a str) -> Self {
        let mut map: HashMap<&'a str, Vec<&'a str>> = HashMap::new();
        let mut broadcast_connections: Vec<&'a str> = Vec::new();

        let mut modules = file
            .lines()
            .filter_map(|line| {
                let (lhs, rhs) = line.split_once(" ->").unwrap();
                let module = Module::new(lhs, rhs);

                if module.mod_type == ModuleType::Broadcast {
                    broadcast_connections = module.connections.clone();
                    return None;
                }

                for name in &module.connections {
                    match map.get_mut(*name) {
                        Some(vector) => {
                            vector.push(&lhs[1..]);
                        }
                        None => {
                            map.insert(*name, vec![&lhs[1..]]);
                        }
                    }
                }

                Some(module)
            })
            .collect::<Vec<Module>>();

        modules.iter_mut().for_each(|module| {
            if module.mod_type == ModuleType::Conjuction {
                module.init_conj_info(map.get(module.conj_info.as_ref().unwrap().name).unwrap());
            }
        });

        Self {
            modules,
            broadcast_connections,
            low_pulse_count: 0,
            high_pulse_count: 0,
        }
    }
    fn dump(&self) {
        for module in &self.modules {
            println!("{:?}", module);
        }
        println!("{:}", self.low_pulse_count);
        println!("{:}", self.high_pulse_count);
    }

    /*
     * Updates current pulse history based on action the it gens
     * future actions
     */
    //fn update_and_send_signal(&mut self, action: Action) -> Option<Vec<Action>> {
    //    let found_module: &mut Module = self
    //        .modules
    //        .iter_mut()
    //        .find(|module| {
    //            if let Some(found) = module.flip_info {
    //                found.name == action.dest
    //            } else if let Some(found) = &module.conj_info {
    //                found.name == action.dest
    //            } else {
    //                panic!("Unable to find module with name");
    //            }
    //        })
    //        .unwrap();

    //    match found_module.mod_type {
    //        ModuleType::FlipFlop => match action.pulse {
    //            Pulse::High => {
    //                return None;
    //            }
    //            Pulse::Low => {
    //                found_module.flip_info.unwrap().flip();
    //                return Some(found_module.gen_actions());
    //            }
    //        },
    //        ModuleType::Conjuction => {
    //            match found_module
    //                .conj_info
    //                .as_mut()
    //                .unwrap()
    //                .received_history
    //                .iter_mut()
    //                .find(|(name, last_pulse)| *name == action.origin)
    //            {
    //                Some((_, last_pulse)) => {
    //                    *last_pulse = action.pulse;
    //                    return Some(found_module.gen_actions());
    //                }
    //                None => {
    //                    panic!("Unable to find action origin in conjuction history");
    //                }
    //            }
    //        }
    //        _ => panic!("How did you not find a name?"),
    //    }
    //}

    fn broadcast(&mut self) {
        let mut actions: Vec<Action> = Vec::new();
        let temp = Module::new_broadcast(self.broadcast_connections.clone());
        actions.append(&mut temp.gen_actions());
        self.low_pulse_count += 1;

        println!("Actions vec start");
        for action in &actions {
            println!("  | {:?}", action);
        }


        while actions.len() != 0 {
            let front = actions.remove(0);

            match front.pulse {
                Pulse::High => {
                    self.high_pulse_count += 1;
                }
                Pulse::Low => {
                    self.low_pulse_count += 1;
                }
            }
            println!("{:}", front.dest);
            let found_module = self
                .modules
                .iter_mut()
                .find(|module| {
                    if let Some(found) = module.flip_info {
                        found.name == front.dest
                    } else if let Some(found) = &module.conj_info {
                        found.name == front.dest
                    } else {
                        panic!("Unable to find module with name");
                    }
                });
            
            //println!("---> Current module {:?}", found_module);
            //println!("---> Current pulse {:?}", front);

            
            if let Some(found_module) = found_module {
                match found_module.update_and_send_signal(front) {
                    Some(mut arr) => {
                        actions.append(&mut arr);
                    }
                    None => (),
                }
            }


            //println!("loop");
            //for action in &actions {
            //    println!("  | {:?}", action);
            //}
            //println!("======================");
        }
        //self.dump();
    }

    fn q1(&mut self) -> u64 {
        (0..1000).for_each(|_| self.broadcast() );
        self.low_pulse_count * self.high_pulse_count
    }
}


fn main() {
    //let file = include_str!("../input/sample.txt");
    let file = include_str!("../input/input.txt");

    let mut module_configuration = ModuleConfiguration::new(file);
    //module_configuration.broadcast();
    println!("{:}", module_configuration.q1());
}

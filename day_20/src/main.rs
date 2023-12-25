use std::collections::HashMap;

#[derive(Debug)]
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

struct Action<'a> {
    pulse: Pulse,
    dest: &'a str,
}

impl<'a> Action<'a> {
    fn new(pulse: Pulse, dest: &'a str) -> Self {
        Self { pulse, dest }
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
}

#[derive(Debug)]
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
    fn activate(&self) -> Vec<Action> {
        match self.mod_type {
            ModuleType::Broadcast => {
                return self
                    .connections
                    .iter()
                    .map(|connection| Action::new(Pulse::Low, connection))
                    .collect::<Vec<Action>>();
            }
            ModuleType::FlipFlop => match self.flip_info.unwrap().current_status {
                Status::On => {
                    todo!();
                }
                Status::Off => {
                    todo!();
                }
            },
            ModuleType::Conjuction => {
                todo!();
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
    modules: Vec<Module<'a>>,
}

impl<'a> ModuleConfiguration<'a> {
    fn new(file: &'a str) -> Self {
        let mut map: HashMap<&'a str, Vec<&'a str>> = HashMap::new();

        let mut modules = file
            .lines()
            .map(|line| {
                let (lhs, rhs) = line.split_once(" ->").unwrap();
                let module = Module::new(lhs, rhs);

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

                module
            })
            .collect::<Vec<Module>>();
        modules.iter_mut().for_each(|module| {
            if module.mod_type == ModuleType::Conjuction {
                module.init_conj_info(map.get(module.conj_info.as_ref().unwrap().name).unwrap());
            }
        });

        Self { modules }
    }
    fn dump(&self) {
        for module in &self.modules {
            println!("{:?}", module);
        }
    }
    fn broadcast(&mut self) {
        let actions: Vec<Action> = Vec::new();

        for module in &self.modules {}
    }
}

fn main() {
    let file = include_str!("../input/sample.txt");
    let module_configuration = ModuleConfiguration::new(file);
    module_configuration.dump();
}

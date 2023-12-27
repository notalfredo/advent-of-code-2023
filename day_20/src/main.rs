use std::collections::HashMap;

fn gcd(a: usize, b: usize) -> usize {
    if b <= 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcd(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

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
    fn new(name: &'a str) -> Self {
        Self {
            name,
            current_status: Status::Off,
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

#[derive(Debug, Clone)]
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
                    flip_info: Some(FlipFlop::new(&lhs[1..])),
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
struct ExtraInfo {
    dest: String,
    high_pulse_index: Vec<(usize, Pulse)>,
    count: usize,
}

impl ExtraInfo {
    fn new(dest: String) -> Self {
        Self {
            dest,
            high_pulse_index: Vec::new(),
            count: 0,
        }
    }
    fn dump(&self) {
        println!("dest: {:}", self.dest);
        for i in 1..self.high_pulse_index.len() {
            println!(
                "index: {:?}, pulse {:?}, diff {:}",
                self.high_pulse_index[i].0,
                self.high_pulse_index[i].1,
                self.high_pulse_index[i].0 - self.high_pulse_index[i - 1].0,
            )
        }
        println!("count: {:}", self.count);
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
    fn new_from_modules(modules: Vec<Module<'a>>, start_loc: &'a str) -> Self {
        Self {
            modules,
            broadcast_connections: vec![start_loc],
            low_pulse_count: 0,
            high_pulse_count: 0,
        }
    }

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
        println!("Broadcast connections {:?}", self.broadcast_connections);
        for module in &self.modules {
            println!("{:?}", module);
        }
        println!("low_pulse_count : {:}", self.low_pulse_count);
        println!("high_pulse_count: {:}", self.high_pulse_count);
    }

    fn broadcast(&mut self, info: &mut Option<ExtraInfo>) {
        let mut actions: Vec<Action> = Vec::new();
        let temp = Module::new_broadcast(self.broadcast_connections.clone());
        actions.append(&mut temp.gen_actions());
        self.low_pulse_count += 1;

        if info.is_some() {
            info.as_mut().unwrap().count += 1;
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

            if info.is_some() {
                if info.as_mut().unwrap().dest == front.dest && front.pulse == Pulse::Low {
                    let temp_button_press_count = info.as_ref().unwrap().count;
                    info.as_mut()
                        .unwrap()
                        .high_pulse_index
                        .push((temp_button_press_count, Pulse::High));
                }
            }

            let found_module = self.modules.iter_mut().find(|module| {
                if let Some(found) = module.flip_info {
                    found.name == front.dest
                } else if let Some(found) = &module.conj_info {
                    found.name == front.dest
                } else {
                    panic!("Unable to find module with name");
                }
            });

            if let Some(found_module) = found_module {
                match found_module.update_and_send_signal(front) {
                    Some(mut arr) => {
                        actions.append(&mut arr);
                    }
                    None => (),
                }
            }
        }
    }

    fn get_sub_graphs(&self) -> (Vec<ModuleConfiguration>, Vec<&str>) {
        let mut module_clone = self.modules.clone();
        let mut name_connections_that_point_to_rx: Vec<&str> = Vec::new();

        //Find node that points to rx
        let pos = module_clone
            .iter()
            .position(|module| {
                module
                    .connections
                    .iter()
                    .find(|name| **name == "rx")
                    .is_some()
            })
            .unwrap();

        let pointer_to_rx = module_clone.remove(pos);

        module_clone.iter().for_each(|module| {
            //This module points to pointer_to_rx
            //We need to keep track of this so we can know
            //when a a high pulse is sent to the naem of pointer_to_rx
            if let Some(_) = module.connections.iter().find(|connection_name| {
                *connection_name == &pointer_to_rx.conj_info.as_ref().unwrap().name
            }) {
                match module.mod_type {
                    ModuleType::FlipFlop => {
                        name_connections_that_point_to_rx
                            .push(module.flip_info.as_ref().unwrap().name);
                    }
                    ModuleType::Conjuction => {
                        name_connections_that_point_to_rx
                            .push(module.conj_info.as_ref().unwrap().name);
                    }
                    _ => panic!("How is this even possible"),
                }
            }
        });

        let mut sub_graphs: Vec<ModuleConfiguration> = Vec::new();

        for start in &self.broadcast_connections {
            let mut sub_graph: Vec<Module> = Vec::new();
            let mut connections_to_find: Vec<&str> = Vec::new();

            connections_to_find.push(start);

            while connections_to_find.len() != 0 {
                let connection_name = connections_to_find.remove(0);

                match module_clone.iter().position(|module| {
                    if let Some(flip) = &module.flip_info {
                        flip.name == connection_name
                    } else if let Some(conj) = &module.conj_info {
                        conj.name == connection_name
                    } else {
                        false
                    }
                }) {
                    Some(found_connection_pos) => {
                        let node_connected_to = module_clone.remove(found_connection_pos);
                        node_connected_to
                            .connections
                            .iter()
                            .for_each(|connection_name| {
                                connections_to_find.push(connection_name);
                            });
                        sub_graph.push(node_connected_to);
                    }
                    //Probably found in a prev iteration
                    None => {
                        continue;
                    }
                }
            }
            sub_graphs.push(ModuleConfiguration::new_from_modules(sub_graph, start));
        }

        (sub_graphs, name_connections_that_point_to_rx)
    }

    fn reset(&mut self) {
        self.modules
            .iter_mut()
            .for_each(|module| match module.mod_type {
                ModuleType::FlipFlop => {
                    module.flip_info.as_mut().unwrap().current_status = Status::Off;
                }
                ModuleType::Conjuction => {
                    module
                        .conj_info
                        .as_mut()
                        .unwrap()
                        .received_history
                        .iter_mut()
                        .for_each(|(_, status)| *status = Pulse::Low);
                }
                _ => (),
            });
    }

    fn q1(&mut self) -> u64 {
        (0..1000).for_each(|_| self.broadcast(&mut None));
        self.low_pulse_count * self.high_pulse_count
    }

    fn q2(&mut self) -> usize {
        let (sub_graphs, layer_one_nodes): (Vec<ModuleConfiguration>, Vec<&str>) =
            self.get_sub_graphs();

        let mut cycles: Vec<usize> = Vec::new();

        for mut sub_graph in sub_graphs {
            let mut extra: Option<ExtraInfo> = None;

            for module in &sub_graph.modules {
                match module.mod_type {
                    ModuleType::FlipFlop => {
                        if let Some(name) = layer_one_nodes
                            .iter()
                            .find(|name| *name == &module.flip_info.as_ref().unwrap().name)
                        {
                            extra = Some(ExtraInfo::new(name.to_string()));
                        } else {
                            continue;
                        }
                    }
                    ModuleType::Conjuction => {
                        if let Some(name) = layer_one_nodes
                            .iter()
                            .find(|name| *name == &module.conj_info.as_ref().unwrap().name)
                        {
                            extra = Some(ExtraInfo::new(name.to_string()));
                        } else {
                            continue;
                        }
                    }
                    _ => (),
                }
            }

            loop {
                sub_graph.broadcast(&mut extra);
                if extra.as_ref().unwrap().high_pulse_index.len() > 0 {
                    cycles.push(extra.as_ref().unwrap().high_pulse_index[0].0);
                    break;
                }
            }
        }

        let mut lcd_calc = lcd(cycles[0], cycles[1]);
        for index in 2..cycles.len() {
            lcd_calc = lcd(lcd_calc, cycles[index]);
        }
        lcd_calc
    }
}

fn main() {
    //let file = include_str!("../input/sample.txt");
    let file = include_str!("../input/input.txt");

    let mut module_configuration = ModuleConfiguration::new(file);
    println!("Q1: {:}", module_configuration.q1());
    module_configuration.reset();
    println!("Q2: {:}", module_configuration.q2());
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn monitor_high_pulse_cycle() {
        let file = include_str!("../input/input.txt");
        let module_configuration = ModuleConfiguration::new(file);
        let (sub_graphs, layer_one_nodes): (Vec<ModuleConfiguration>, Vec<&str>) =
            module_configuration.get_sub_graphs();

        for mut sub_graph in sub_graphs {
            let mut extra: Option<ExtraInfo> = None;

            //Try to find what layer one node this sub graph contains
            for module in &sub_graph.modules {
                match module.mod_type {
                    ModuleType::FlipFlop => {
                        if let Some(name) = layer_one_nodes
                            .iter()
                            .find(|name| *name == &module.flip_info.as_ref().unwrap().name)
                        {
                            extra = Some(ExtraInfo::new(name.to_string()));
                        } else {
                            continue;
                        }
                    }
                    ModuleType::Conjuction => {
                        if let Some(name) = layer_one_nodes
                            .iter()
                            .find(|name| *name == &module.conj_info.as_ref().unwrap().name)
                        {
                            extra = Some(ExtraInfo::new(name.to_string()));
                        } else {
                            continue;
                        }
                    }
                    _ => (),
                }
            }

            (0..100000).for_each(|_| sub_graph.broadcast(&mut extra));

            println!("=============");
            sub_graph.dump();
            extra.unwrap().dump();
            println!("=============");
        }
    }
}

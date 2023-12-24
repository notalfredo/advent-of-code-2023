use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Rating {
    X,
    M,
    A,
    S,
}

impl From<char> for Rating {
    fn from(rating: char) -> Self {
        match rating {
            'x' => return Rating::X,
            'm' => return Rating::M,
            'a' => return Rating::A,
            's' => return Rating::S,
            _ => panic!("Trying to convert rating for non x/m/a/s char"),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Location<'a> {
    Label(&'a str),
    Accept,
    Reject,
}

impl<'a> Location<'a> {
    fn is_label(&self) -> bool {
        match self {
            Location::Label(_) => true,
            _ => false,
        }
    }
    fn is_accept(&self) -> bool {
        match self {
            Location::Accept => true,
            _ => false,
        }
    }
    fn is_reject(&self) -> bool {
        match self {
            Location::Reject => true,
            _ => false,
        }
    }
}

impl<'a> From<&'a str> for Location<'a> {
    fn from(location: &'a str) -> Self {
        match location {
            "A" => return Location::Accept,
            "R" => return Location::Reject,
            _ => return Location::Label(location),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Lt,
    Gt,
}

impl Operator {
    fn compare(&self, lhs: u32, rhs: u32) -> bool {
        match self {
            Operator::Lt => lhs < rhs,
            Operator::Gt => lhs > rhs,
        }
    }
    fn flip(&self) -> Operator {
        match *self {
            Operator::Lt => {
                return Operator::Gt;
            }
            Operator::Gt => {
                return Operator::Lt;
            }
        }
    }
}

#[derive(Debug)]
struct Variable {
    name: Rating,
    value: u32,
}

impl Variable {
    fn new(name: Rating, value: u32) -> Variable {
        Variable { name, value }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rule<'a> {
    lhs: Option<Rating>,
    op: Option<Operator>,
    rhs: Option<u32>,
    location: Location<'a>,
}

impl<'a> Rule<'a> {
    fn new(rule: &'a str) -> Rule {
        match rule.split_once(':') {
            Some((predicate, location)) => match predicate.split_once('<') {
                Some((lhs, rhs)) => {
                    return Rule {
                        lhs: Some(Rating::from(lhs.chars().next().unwrap())),
                        op: Some(Operator::Lt),
                        rhs: Some(rhs.parse::<u32>().unwrap()),
                        location: Location::from(location),
                    }
                }
                None => {
                    let (lhs, rhs) = predicate.split_once('>').unwrap();
                    return Rule {
                        lhs: Some(Rating::from(lhs.chars().next().unwrap())),
                        op: Some(Operator::Gt),
                        rhs: Some(rhs.parse::<u32>().unwrap()),
                        location: Location::from(location),
                    };
                }
            },
            None => {
                return Rule {
                    lhs: None,
                    op: None,
                    rhs: None,
                    location: Location::from(rule),
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
}

struct DfsNode {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
    sum: u64,
    debug: u32,
}

impl DfsNode {
    fn new() -> Self {
        Self {
            x: Range::new(1, 4000),
            m: Range::new(1, 4000),
            a: Range::new(1, 4000),
            s: Range::new(1, 4000),
            debug: 0,
            sum: 0,
        }
    }
    fn dump(&self) {
        println!("x {:?}", self.x);
        println!("m {:?}", self.m);
        println!("a {:?}", self.a);
        println!("s {:?}", self.s);
        println!("sum {:?}", self.sum);
    }


    fn add_combinations(&mut self) {
        let mut combinations = 1;
        combinations *= self.x.end - (self.x.start - 1);
        combinations *= self.m.end - (self.m.start - 1);
        combinations *= self.a.end - (self.a.start - 1);
        combinations *= self.s.end - (self.s.start - 1);
        println!("Going to add {:}", combinations);

        self.sum += combinations;
        self.debug += 1;
    }
    fn get_x(&self) -> Range {
        self.x
    }
    fn get_m(&self) -> Range {
        self.m
    }
    fn get_a(&self) -> Range {
        self.a
    }
    fn get_s(&self) -> Range {
        self.s
    }

    fn make_rule_true(&mut self, lhs: Rating, op: Operator, rhs: u64) {
        match lhs {
            Rating::X => match op {
                Operator::Gt => {
                    self.x.start = rhs + 1;
                }
                Operator::Lt => {
                    self.x.end = rhs - 1;
                }
            },
            Rating::M => match op {
                Operator::Gt => {
                    self.m.start = rhs + 1;
                }
                Operator::Lt => {
                    self.m.end = rhs - 1;
                }
            },
            Rating::A => match op {
                Operator::Gt => {
                    self.a.start = rhs + 1;
                }
                Operator::Lt => {
                    self.a.end = rhs - 1;
                }
            },
            Rating::S => match op {
                Operator::Gt => {
                    self.s.start = rhs + 1;
                }
                Operator::Lt => {
                    self.s.end = rhs - 1;
                }
            },
        }
    }

    fn restore(&mut self, x: Range, m: Range, a: Range, s: Range) {
        self.x = x;
        self.m = m;
        self.a = a;
        self.s = s;
    }

}

struct Workflow<'a> {
    workflows: HashMap<Location<'a>, Vec<Rule<'a>>>,
    ratings: Vec<[Variable; 4]>,
}

impl<'a> Workflow<'a> {
    fn new(file: &'a str) -> Self {
        let (workflows, ratings) = file.split_once("\n\n").unwrap();
        let workflows: Vec<(Location, Vec<Rule>)> = workflows
            .lines()
            .map(|workflows: &str| {
                let (workflow_name, rules) = workflows.split_once("{").unwrap();
                let rules: Vec<&str> = (&rules[0..rules.len() - 1]).split(',').collect();
                let rules: Vec<Rule> = rules.iter().map(|rule| Rule::new(rule)).collect();
                let workflow_name = Location::from(workflow_name);

                (workflow_name, rules)
            })
            .collect();
        let workflows: HashMap<Location, Vec<Rule>> = workflows.into_iter().collect();

        let ratings: Vec<[Variable; 4]> = ratings
            .lines()
            .map(|rating: &str| {
                let trim = &rating[1..rating.len() - 1];
                let assignments: [Variable; 4] = trim
                    .split(',')
                    .map(|assignment| {
                        let (variable, value) = assignment.split_once('=').unwrap();
                        let variable = Rating::from(variable.chars().next().unwrap());
                        let value = value.parse::<u32>().unwrap();
                        Variable::new(variable, value)
                    })
                    .collect::<Vec<Variable>>()
                    .try_into()
                    .unwrap();
                assignments
            })
            .collect();

        Self { workflows, ratings }
    }

    fn dump(&self) {
        for (label_name, rules) in &self.workflows {
            println!("{:?}, {:?}", label_name, rules);
        }
        for row in &self.ratings {
            println!("{:?}", row);
        }
    }

    fn check_row(&self, row_num: usize) -> u32 {
        let mut current_rule = &self.workflows[&Location::from("in")];

        let binding = &Location::from("in");
        let mut visted_rules: Vec<&Location> = vec![binding];

        loop {
            //println!("current_rule {:?}", current_rule);
            for rule in current_rule {
                //println!("{:?}", visted_rules);
                //println!("      | Looking at{:?}", rule);
                match &rule.lhs {
                    Some(left_hand_side) => {
                        //println!("          | Rule has something, {:?} {:?} {:?}",
                        //                                            left_hand_side,
                        //                                            &rule.op,
                        //                                            &rule.rhs
                        //                                            );
                        match self.ratings[row_num]
                            .iter()
                            .find(|var| var.name == *left_hand_side)
                        {
                            Some(var) => {
                                //println!("          | var found {:?}", var);
                                if rule
                                    .op
                                    .as_ref()
                                    .unwrap()
                                    .compare(var.value, rule.rhs.unwrap())
                                {
                                    //println!("          | comparison passed {:?}", var);
                                    if rule.location.is_label() {
                                        //println!("          | found label {:?}", rule.location);

                                        visted_rules.push(&rule.location);
                                        current_rule = &self.workflows[&rule.location];
                                        break;
                                    } else if rule.location.is_accept() {
                                        //println!("          | accepted {:?}", var);
                                        return self.ratings[row_num]
                                            .iter()
                                            .map(|rating| rating.value)
                                            .sum::<u32>();
                                    } else if rule.location.is_reject() {
                                        return 0;
                                    }
                                }
                                //println!("          | comparison did not passed");
                            }
                            None => {
                                continue;
                            }
                        }
                    }
                    None => {
                        //println!("           | DID NOT FIND ANY PREDICATE");
                        if rule.location.is_label() {
                            //println!("          | found label {:?}", rule.location);
                            visted_rules.push(&rule.location);
                            current_rule = &self.workflows[&rule.location];
                            break;
                        } else if rule.location.is_accept() {
                            //println!("           | Accepted");
                            return self.ratings[row_num]
                                .iter()
                                .map(|rating| rating.value)
                                .sum::<u32>();
                        }
                        //println!("           | Rejected");
                        return 0;
                    }
                }
            }
        }
    }

    fn dfs(&self, node: &mut DfsNode, workflow: &Vec<Rule>) {
        println!("looking at workflow: {:?}", workflow);
        node.dump();
        for rule in workflow {
            println!("      | looking at rule {:?}", rule);
            match &rule.lhs {
                //We found a predicate, make it true and recurse
                //make it false and continue to the right
                Some(_) => {
                    println!("      | PREDICATE FOUND");
                    match rule.location {
                        Location::Label(_) => {
                            println!("Label found"); 
                            let (x, m, a, s) =
                                (node.get_x(), node.get_m(), node.get_a(), node.get_s());

                            //TODO: Make predicate true and Recurse here             
                            node.make_rule_true(rule.lhs.unwrap(), rule.op.unwrap(), rule.rhs.unwrap() as u64);
                            self.dfs(node, &self.workflows[&rule.location].clone());

                            node.restore(x, m, a, s);
                            //TODO: Make predicate false and continue
                            node.make_rule_true(rule.lhs.unwrap(), rule.op.unwrap().flip(), rule.rhs.unwrap() as u64);
                        },
                        Location::Accept => {
                            let (x, m, a, s) =
                                (node.get_x(), node.get_m(), node.get_a(), node.get_s());
                            println!("-------> ACCEPTED <-----------");
                            node.make_rule_true(rule.lhs.unwrap(), rule.op.unwrap(), rule.rhs.unwrap() as u64);
                            println!("looking at workflow: {:?}", workflow);
                            node.dump();
                            println!("-------> ACCEPTED <-----------");
                            node.add_combinations();
                            node.restore(x, m, a, s);
                            //TODO: Make predicate false and continue
                            node.make_rule_true(rule.lhs.unwrap(), rule.op.unwrap().flip(), rule.rhs.unwrap() as u64);
                        },
                        Location::Reject => {
                            node.make_rule_true(rule.lhs.unwrap(), rule.op.unwrap().flip(), rule.rhs.unwrap() as u64);
                            println!("-------> REJECT <-----------");
                            continue;
                        }
                    }
                }
                //We are at the final rule, we either found a
                //new label, accepted or rejected. If we found a label
                //recurse down again
                None => {
                    println!("      | PREDICATE NOT FOUND");
                    match rule.location {
                        Location::Label(_) => {
                            self.dfs(node, &self.workflows[&rule.location].clone());
                        },
                        Location::Accept => {
                            println!("-------> ACCEPTED");
                            node.dump();
                            println!("-------> ACCEPTED");
                            node.add_combinations();
                        },
                        Location::Reject => {
                            println!("-------> Rejected");
                            return;
                        }
                    }
                }
            }
        }
    }

    fn q1(&self) {
        println!(
            "Q1: {:}",
            (0..self.ratings.len())
                .map(|num| { self.check_row(num) })
                .sum::<u32>()
        );
    }

    fn q2(&self) {
        let mut node = DfsNode::new();
        let current_rule = self.workflows[&Location::from("in")].clone();
        self.dfs(&mut node, &current_rule);
        println!("Q2 {:}", node.sum);
        println!("Q2 {:}", node.debug);
    }
}

fn main() {
    let file = include_str!("../input/sample.txt");
    //let file = include_str!("../input/input.txt");
    //let file = include_str!("../input/sample_two.txt");

    let workflow = Workflow::new(file);
    //workflow.q1();
    workflow.q2();
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn assert_length() {
        let file = include_str!("../input/sample.txt");
        let workflow = Workflow::new(file);

        assert_eq!(workflow.workflows.len(), 11);
        assert_eq!(workflow.ratings.len(), 5);

        let file = include_str!("../input/input.txt");
        let workflow = Workflow::new(file);

        assert_eq!(workflow.workflows.len(), 569);
        assert_eq!(workflow.ratings.len(), 200);
    }
}

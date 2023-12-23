use std::collections::HashMap;

#[derive(Debug)]
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

#[derive(Debug, Hash, Eq, PartialEq)]
enum Location<'a> {
    Label(&'a str),
    Accept,
    Reject,
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

#[derive(Debug)]
enum Operator {
    Le,
    Gt,
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

#[derive(Debug)]
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
                        op: Some(Operator::Le),
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

struct Workflow<'a> {
    workflows: HashMap<Location<'a>, Vec<Rule<'a>>>,
    ratings: Vec<[Variable; 4]>,
}

impl<'a> Workflow<'a> {
    fn new(file: &'a str) -> Self {
        let (workflows, ratings) = file.split_once("\n\n").unwrap();
        let workflows: Vec<(Location<'a>, Vec<Rule>)> = workflows
            .lines()
            .map(|workflows: &str| {
                let (workflow_name, rules) = workflows.split_once("{").unwrap();
                let rules: Vec<&'a str> = (&rules[0..rules.len() - 1]).split(',').collect();
                let rules: Vec<Rule> = rules.iter().map(|rule| Rule::new(rule)).collect();
                let workflow_name = Location::from(workflow_name);

                (workflow_name, rules)
            })
            .collect();
        let workflows: HashMap<Location, Vec<Rule<'a>>> = workflows.into_iter().collect();

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

}

fn main() {
    let file = include_str!("../input/sample.txt");

    let workflow = Workflow::new(file);
    workflow.dump();
}

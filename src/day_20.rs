use std::collections::{HashMap, VecDeque};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("20_example_1.txt").unwrap();
    println!("pt1 first example: {}", pt1(&input));
    let input = fs::read_to_string("20_example_2.txt").unwrap();
    println!("pt1 second example: {}", pt1(&input));
    let input = fs::read_to_string("20.txt").unwrap();
    println!("pt1: {}", pt1(&input));
    println!("pt2: {}", pt2(&input));
}

fn pt1(input: &str) -> u64 {
    let mut broker = PulseBroker::new(input);

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        let pulse = Pulse::Low(String::from("button"), String::from("broadcaster"));
        let (low_pulse_count, high_pulse_count, _) = broker.send_pulse(pulse);
        low_pulses += low_pulse_count;
        high_pulses += high_pulse_count;
    }

    low_pulses * high_pulses
}

fn pt2(input: &str) -> u64 {
    let mut broker = PulseBroker::new(input);

    let mut button_count = 0;

    let mut lh = (0, 0);
    let mut lh_repetition = 0;

    let mut fk = (0, 0);
    let mut fk_repetition = 0;

    let mut ff = (0, 0);
    let mut ff_repetition = 0;

    let mut mm = (0, 0);
    let mut mm_repetition = 0;

    while lh_repetition == 0 || fk_repetition == 0 || ff_repetition == 0 || mm_repetition == 0 {
        button_count += 1;
        let pulse = Pulse::Low(String::from("button"), String::from("broadcaster"));
        let (_, _, end_conjunctions_state) = broker.send_pulse(pulse);
        if end_conjunctions_state.0 {
            let cycle_count = button_count - lh.0;
            lh.0 = button_count;
            if lh.1 == cycle_count {
                lh_repetition = cycle_count;
            }
            lh.1 = cycle_count;
        }
        if end_conjunctions_state.1 {
            let cycle_count = button_count - fk.0;
            fk.0 = button_count;
            if fk.1 == cycle_count {
                fk_repetition = cycle_count;
            }
            fk.1 = cycle_count;
        }
        if end_conjunctions_state.2 {
            let cycle_count = button_count - ff.0;
            ff.0 = button_count;
            if ff.1 == cycle_count {
                ff_repetition = cycle_count;
            }
            ff.1 = cycle_count;
        }
        if end_conjunctions_state.3 {
            let cycle_count = button_count - mm.0;
            mm.0 = button_count;
            if mm.1 == cycle_count {
                mm_repetition = cycle_count;
            }
            mm.1 = cycle_count;
        }
    }

    let mut answer = 1;

    for repetition in [lh_repetition, fk_repetition, ff_repetition, mm_repetition] {
        answer = lcm(answer, repetition);
    }

    answer
}

// (source, destination)
#[derive(Debug)]
enum Pulse {
    Low(String, String),
    High(String, String),
}

struct FlipFlop {
    name: String,
    on: bool,
    destination_modules: Vec<String>,
}

impl FlipFlop {
    fn new(name: String, destination_modules: Vec<String>) -> Self {
        Self {
            name,
            on: false,
            destination_modules,
        }
    }
}

struct Conjunction {
    name: String,
    input_module_memory: HashMap<String, bool>,
    destination_modules: Vec<String>,
}

impl Conjunction {
    fn new(name: String, destination_modules: Vec<String>, input_modules: &Vec<String>) -> Self {
        let mut input_module_memory = HashMap::new();
        for input_module in input_modules {
            input_module_memory.insert(input_module.to_string(), false);
        }
        Self {
            name,
            input_module_memory,
            destination_modules,
        }
    }
}

struct Broadcaster {
    destination_modules: Vec<String>,
}

impl Broadcaster {
    fn new(destination_modules: Vec<String>) -> Self {
        Self {
            destination_modules,
        }
    }
}

trait Module {
    // returns Vec of Pulses the module sends in response
    fn receive_pulse(&mut self, pulse: Pulse) -> Vec<Pulse>;

    fn register_input_modules(&mut self, _: HashMap<String, Box<dyn Module>>) {}
}

impl Module for FlipFlop {
    fn receive_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        if let Pulse::Low(_, _) = pulse {
            self.on = !self.on;
            match self.on {
                true => self
                    .destination_modules
                    .iter()
                    .map(|destination| {
                        Pulse::High(String::from(&self.name), destination.to_string())
                    })
                    .collect(),
                false => self
                    .destination_modules
                    .iter()
                    .map(|destination| {
                        Pulse::Low(String::from(&self.name), destination.to_string())
                    })
                    .collect(),
            }
        } else {
            vec![]
        }
    }
}

impl Module for Conjunction {
    fn receive_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        match pulse {
            Pulse::High(source, _) => {
                self.input_module_memory.insert(source, true);
            }
            Pulse::Low(source, _) => {
                self.input_module_memory.insert(source, false);
            }
        }

        if self.input_module_memory.values().all(|v| *v) {
            // all input modules have sent High pulses, so return Low pulses to be sent
            self.destination_modules
                .iter()
                .map(|destination| Pulse::Low(String::from(&self.name), destination.to_string()))
                .collect()
        } else {
            // at least one input module has sent a Low pulse, so return High pulses to be sent
            self.destination_modules
                .iter()
                .map(|destination| Pulse::High(String::from(&self.name), destination.to_string()))
                .collect()
        }
    }
}

impl Module for Broadcaster {
    fn receive_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        match pulse {
            Pulse::High(_, _) => self
                .destination_modules
                .iter()
                .map(|destination| {
                    Pulse::High(String::from("broadcaster"), destination.to_string())
                })
                .collect(),
            Pulse::Low(_, _) => self
                .destination_modules
                .iter()
                .map(|destination| Pulse::Low(String::from("broadcaster"), destination.to_string()))
                .collect(),
        }
    }
}

struct PulseBroker {
    broadcaster: Broadcaster,
    modules: HashMap<String, Box<dyn Module>>,
    queue: VecDeque<Pulse>,
}

impl PulseBroker {
    fn new(input: &str) -> Self {
        let parsed_input: Vec<_> = input
            .lines()
            .map(|line| {
                let (module_name, destination_modules_str) = line.split_once(" -> ").unwrap();

                let destination_modules: Vec<_> = destination_modules_str.split(", ").collect();

                if &module_name[..1] == "&" {
                    ("conjunction", &module_name[1..], destination_modules)
                } else if &module_name[..1] == "%" {
                    ("flipflop", &module_name[1..], destination_modules)
                } else {
                    ("broadcaster", "broadcaster", destination_modules)
                }
            })
            .collect();

        let mut input_lookup: HashMap<&str, Vec<&str>> = HashMap::new();

        for (_, module_name, destination_modules) in &parsed_input {
            input_lookup.entry(module_name).or_insert(vec![]);
            for destination_module in destination_modules {
                let input_modules = input_lookup.entry(destination_module).or_insert(vec![]);
                input_modules.push(module_name);
            }
        }

        // println!("{:?}", parsed_input);
        // println!("{:?}", input_lookup);

        let mut broadcaster = None;
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

        for (module_type, module_name, destination_modules) in parsed_input {
            let mut string_destination_modules = vec![];

            for module_name in destination_modules {
                string_destination_modules.push(String::from(module_name));
            }

            match module_type {
                "broadcaster" => broadcaster = Some(Broadcaster::new(string_destination_modules)),
                "conjunction" => {
                    let input_modules = input_lookup.get(module_name).unwrap();

                    let mut string_input_modules = vec![];

                    for module_name in input_modules {
                        string_input_modules.push(String::from(*module_name));
                    }

                    let module = Box::new(Conjunction::new(
                        module_name.to_string(),
                        string_destination_modules,
                        &string_input_modules,
                    ));
                    modules.insert(String::from(module_name), module);
                }
                "flipflop" => {
                    let module = Box::new(FlipFlop::new(
                        module_name.to_string(),
                        string_destination_modules,
                    ));
                    modules.insert(String::from(module_name), module);
                }
                _ => panic!("Invalid module type: {module_type}"),
            }
        }

        let queue = VecDeque::new();

        match broadcaster {
            Some(broadcaster) => Self {
                modules,
                queue,
                broadcaster,
            },
            None => panic!("broadcaster not found in input"),
        }
    }

    fn send_pulse(&mut self, pulse: Pulse) -> (u64, u64, (bool, bool, bool, bool)) {
        let mut high_pulse_count = 0;
        let mut low_pulse_count = 0;
        let mut end_conjunctions_state = (false, false, false, false);

        match pulse {
            Pulse::Low(_, _) => low_pulse_count += 1,
            Pulse::High(_, _) => high_pulse_count += 1,
        }

        for pulse in self.broadcaster.receive_pulse(pulse) {
            self.queue.push_back(pulse);
        }

        while let Some(pulse) = self.queue.pop_front() {
            match pulse {
                Pulse::Low(_, _) => low_pulse_count += 1,
                Pulse::High(_, _) => high_pulse_count += 1,
            }

            let destination_module = match &pulse {
                Pulse::Low(_, dest) => dest,
                Pulse::High(_, dest) => dest,
            };

            let module_name = String::from(destination_module);

            if let Some(module) = self.modules.get_mut(destination_module) {
                let output_pulses = module.receive_pulse(pulse);

                let all_high_pulse = output_pulses.iter().all(|pulse| {
                    if let Pulse::High(_, _) = pulse {
                        true
                    } else {
                        false
                    }
                });

                if all_high_pulse {
                    match &module_name[..] {
                        "lh" => {
                            end_conjunctions_state.0 = true;
                        }
                        "fk" => {
                            end_conjunctions_state.1 = true;
                        }
                        "ff" => {
                            end_conjunctions_state.2 = true;
                        }
                        "mm" => {
                            end_conjunctions_state.3 = true;
                        }
                        _ => (),
                    }
                }

                for output_pulse in output_pulses {
                    self.queue.push_back(output_pulse);
                }
            }
        }

        (low_pulse_count, high_pulse_count, end_conjunctions_state)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

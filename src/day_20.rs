use std::collections::{HashMap, VecDeque};
use std::fs;

pub fn run() {
    let example_input = fs::read_to_string("20_example_1.txt").unwrap();
    println!("pt1 first example: {}", pt1(&example_input));
    let example_input = fs::read_to_string("20_example_2.txt").unwrap();
    println!("pt1 second example: {}", pt1(&example_input));
}

fn pt1(input: &str) -> u64 {
    let mut broker = PulseBroker::new(input);
    println!("{:?}", broker.modules.keys());

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        let pulse = Pulse::Low(String::from("button"), String::from("broadcaster"));
        let (low_pulse_count, high_pulse_count) = broker.send_pulse(pulse);
        low_pulses += low_pulse_count;
        high_pulses += high_pulse_count;
    }

    low_pulses * high_pulses
}

// (source, destination)
enum Pulse {
    Low(String, String),
    High(String, String),
}

struct FlipFlop{
    name: String,
    on: bool,
    destination_modules: Vec<String>,
}

impl FlipFlop{
    fn new(name: String, destination_modules: Vec<String>) -> Self {
        Self { name, on: false, destination_modules }
    }
}

struct Conjunction{
    name: String,
    input_module_memory: HashMap<String, bool>,
    destination_modules: Vec<String>,
}

impl Conjunction {
    fn new(name: String, destination_modules: Vec<String>) -> Self {
        Self { name, input_module_memory: HashMap::new(), destination_modules }
    }
}

struct Broadcaster{
    destination_modules: Vec<String>,
}

impl Broadcaster{
    fn new(destination_modules: Vec<String>) -> Self {
        Self { destination_modules }
    }
}

trait Module {
    // returns Vec of Pulses the module sends in response
    fn receive_pulse(&mut self, pulse: Pulse) -> Vec<Pulse>;
}

impl Module for FlipFlop {
    fn receive_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        if let Pulse::Low(_, _) = pulse {
            self.on = !self.on;
            match self.on {
                true => self.destination_modules.iter().map(|destination| Pulse::High(String::from(&self.name), destination.to_string())).collect(),
                false => self.destination_modules.iter().map(|destination| Pulse::Low(String::from(&self.name), destination.to_string())).collect(),
            }
        } else {
            vec![]
        }

    }
}

impl Module for Conjunction{
    fn receive_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        match pulse {
            Pulse::High(source, _) => {
                self.input_module_memory.insert(source, true);
            },
            Pulse::Low(source, _) => {
                self.input_module_memory.insert(source, false);
            },
        }

        if self.input_module_memory.values().all(|v| *v) {
            // all input modules have sent High pulses, so return Low pulses to be sent
            self.destination_modules.iter().map(|destination| Pulse::Low(String::from(&self.name), destination.to_string())).collect()
        } else {
            // at least one input module has sent a Low pulse, so return High pulses to be sent
            self.destination_modules.iter().map(|destination| Pulse::High(String::from(&self.name), destination.to_string())).collect()
        }
    }
}

impl Module for Broadcaster {
    fn receive_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        match pulse {
            Pulse::High(_, _) => self.destination_modules.iter().map(|destination| Pulse::High(String::from("broadcaster"), destination.to_string())).collect(),
            Pulse::Low(_, _) => self.destination_modules.iter().map(|destination| Pulse::Low(String::from("broadcaster"), destination.to_string())).collect(),
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
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

        let mut broadcaster = None;

        for line in input.lines() {
            let (module_name, destination_modules) = line.split_once(" -> ").unwrap();
            let destination_modules = destination_modules.split(", ").map(String::from).collect();
            if module_name == "broadcaster" {
                broadcaster = Some(Broadcaster::new(destination_modules));
            } else if &module_name[..1] == "&" {
                let module_name = &module_name[1..];
                modules.insert(String::from(module_name), Box::new(Conjunction::new(module_name.to_string(), destination_modules)));
            } else if &module_name[..1] == "%" {
                let module_name = &module_name[1..];
                modules.insert(String::from(module_name), Box::new(FlipFlop::new(module_name.to_string(), destination_modules)));
            } else {
                panic!("invalid module name");
            }
        }

        let queue = VecDeque::new();

        match broadcaster {
            Some(broadcaster) => Self { modules, queue, broadcaster },
            None => panic!("broadcaster not found in input"),
        }
    }

    fn send_pulse(&mut self, pulse: Pulse) -> (u64, u64) {
        let mut high_pulse_count = 0;
        let mut low_pulse_count = 0;

        match pulse {
            Pulse::Low(_,_) => low_pulse_count += 1,
            Pulse::High(_,_) => high_pulse_count += 1,
        }

        for pulse in self.broadcaster.receive_pulse(pulse) {
            self.queue.push_back(pulse);
        }

        while let Some(pulse) = self.queue.pop_front() {
            match pulse {
                Pulse::Low(_,_) => low_pulse_count += 1,
                Pulse::High(_,_) => high_pulse_count += 1,
            }

            let destination_module = match &pulse {
                Pulse::Low(_, dest) => dest,
                Pulse::High(_, dest) => dest,
            };

            if let Some(module) = self.modules.get_mut(destination_module) {
                let output_pulses = module.receive_pulse(pulse);

                for output_pulse in output_pulses {
                    self.queue.push_back(output_pulse);
                }
            } else {
                println!("Ignoring module: {}", destination_module);
            }; 
            
        }

        (low_pulse_count, high_pulse_count)
    }
}

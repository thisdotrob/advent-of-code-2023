use std::collections::{HashMap, VecDeque};
use std::fs;

pub fn run() {
    let example_input = fs::read_to_string("20_example.txt").unwrap();
    println!("pt1 example: {}", pt1(&example_input));
}

fn pt1(input: &str) -> u64 {
    let broker = PulseBroker::new(input);
    0
}

// (source, destination)
enum Pulse<'a> {
    Low(&'a str, &'a str),
    High(&'a str, &'a str),
}

struct FlipFlop<'a> {
    name: &'a str,
    on: bool,
    destination_modules: Vec<&'a str>,
}

impl<'a> FlipFlop<'a> {
    fn new(name: &'a str, destination_modules: Vec<&'a str>) -> Self {
        Self { name, on: false, destination_modules }
    }
}

struct Conjunction<'a> {
    name: &'a str,
    input_module_memory: HashMap<&'a str, bool>,
    destination_modules: Vec<&'a str>,
}

impl<'a> Conjunction<'a> {
    fn new(name: &'a str, destination_modules: Vec<&'a str>) -> Self {
        Self { name, input_module_memory: HashMap::new(), destination_modules }
    }
}

struct Broadcaster<'a> {
    destination_modules: Vec<&'a str>,
}

impl<'a> Broadcaster<'a> {
    fn new(destination_modules: Vec<&'a str>) -> Self {
        Self { destination_modules }
    }
}

trait Module<'a> {
    // returns Vec of Pulses the module sends in response
    fn receive_pulse(&'a mut self, pulse: Pulse<'a>) -> Vec<Pulse>;
}

impl<'a> Module<'a> for FlipFlop<'a> {
    fn receive_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        if let Pulse::Low(_, _) = pulse {
            self.on = !self.on;
            match self.on {
                true => self.destination_modules.iter().map(|destination| Pulse::High(self.name, destination)).collect(),
                false => self.destination_modules.iter().map(|destination| Pulse::Low(self.name, destination)).collect(),
            }
        } else {
            vec![]
        }

    }
}

impl<'a> Module<'a> for Conjunction<'a> {
    fn receive_pulse(&mut self, pulse: Pulse<'a>) -> Vec<Pulse> {
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
            self.destination_modules.iter().map(|destination| Pulse::Low(self.name, destination)).collect()
        } else {
            // at least one input module has sent a Low pulse, so return High pulses to be sent
            self.destination_modules.iter().map(|destination| Pulse::High(self.name, destination)).collect()
        }
    }
}

impl<'a> Module<'a> for Broadcaster<'a> {
    fn receive_pulse(&mut self, pulse: Pulse<'a>) -> Vec<Pulse> {
        match pulse {
            Pulse::High(_, _) => self.destination_modules.iter().map(|destination| Pulse::High("broadcaster", destination)).collect(),
            Pulse::Low(_, _) => self.destination_modules.iter().map(|destination| Pulse::Low("broadcaster", destination)).collect(),
        }
    }
}

struct PulseBroker<'a> {
    broadcaster: Broadcaster<'a>,
    modules: HashMap<&'a str, Box<dyn Module<'a> + 'a>>,
    queue: VecDeque<Pulse<'a>>,
}

impl<'a> PulseBroker<'a> {
    fn new(input: &'a str) -> Self {
        let mut modules: HashMap<&str, Box<dyn Module>> = HashMap::new();

        let mut broadcaster = None;

        for line in input.lines() {
            let (module_name, destination_modules) = line.split_once(" -> ").unwrap();
            let destination_modules = destination_modules.split(", ").collect();
            if module_name == "broadcaster" {
                broadcaster = Some(Broadcaster::new(destination_modules));
            } else if &module_name[..1] == "&" {
                let module_name = &module_name[1..];
                modules.insert(module_name, Box::new(Conjunction::new(module_name, destination_modules)));
            } else if &module_name[..1] == "%" {
                modules.insert(module_name, Box::new(FlipFlop::new(module_name, destination_modules)));
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

    fn send_pulse(&'a mut self, pulse: Pulse<'a>) -> u64 {
        for pulse in self.broadcaster.receive_pulse(pulse) {
            self.queue.push_back(pulse);
        }

        if let Some(pulse) = self.queue.pop_front() {
            let destination_module = match pulse {
                Pulse::Low(_, dest) => dest,
                Pulse::High(_, dest) => dest,
            };

            let module = self.modules.get_mut(destination_module).unwrap(); 

            for output_pulse in module.receive_pulse(pulse) {
                self.queue.push_back(output_pulse);
            }
        }

        if let Some(pulse) = self.queue.pop_front() {
            let destination_module = match pulse {
                Pulse::Low(_, dest) => dest,
                Pulse::High(_, dest) => dest,
            };

            let module = self.modules.get_mut(destination_module).unwrap(); 

            for output_pulse in module.receive_pulse(pulse) {
                self.queue.push_back(output_pulse);
            }
        }
        0
    }
}

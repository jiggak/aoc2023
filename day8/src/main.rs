use std::{
    {env, fs, process},
    collections::HashMap
};

use num::Integer;

#[derive(Debug)]
enum Instruction {
    Left,
    Right
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            x => panic!("Instruction: Unhandled char '{x}'")
        }
    }
}

struct Network {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, (String, String)>
}

impl Network {
    fn load(data: &str) -> Self {
        let mut lines = data.lines();

        let inst = lines.next().unwrap();
        let instructions = inst.chars()
            .map(|c| c.into())
            .collect();

        let mut nodes = HashMap::new();

        for line in lines {
            // skip empty lines
            if line.is_empty() {
                continue;
            }

            let mut parts = line.split(" = ");
            let label = parts.next().expect("node label");
            let next_nodes = parts.next().expect("next nodes");

            let mut parts = next_nodes.split(", ");

            nodes.insert(label.into(), (
                parts.next().expect("node left label")[1..].into(),
                parts.next().expect("node right label")[..3].into()
            ));
        }

        Network { instructions, nodes }
    }

    fn count_steps(&self, start: &str, end: &str) -> usize {
        let mut count = 0;
        let mut next = start;

        for inst in self.instructions.iter().cycle() {
            count += 1;

            let node = self.nodes.get(next).unwrap();
            next = match inst {
                Instruction::Left => node.0.as_str(),
                Instruction::Right => node.1.as_str()
            };

            if next.ends_with(end) {
                break;
            }
        }

        count
    }

    // This seems to loop forever. According to what people are saying online,
    // it might be possible to brute force the solution, but I didn't run
    // this long enought to find out if it will work/finish.
    fn _count_steps_multi_start<'a>(&'a self, start: &mut [&'a str], end: &str) -> usize {
        let mut count = 0;

        let next = start;

        for inst in self.instructions.iter().cycle() {
            count += 1;

            for label in next.iter_mut() {
                let node = self.nodes.get(*label).unwrap();
                *label = match inst {
                    Instruction::Left => node.0.as_ref(),
                    Instruction::Right => node.1.as_ref()
                }
            }

            if next.iter().all(|n| n.ends_with(end)) {
                break;
            }
        }

        count
    }
}

fn main() {
    let mut args = env::args();

    // first arg is command name
    let cmd_name = args.next().unwrap();

    let mut run_part2 = false;
    let input_file = match args.next() {
        Some(a) if a == "-p2" => {
            run_part2 = true;
            args.next()
        },
        Some(a) => Some(a),
        None => None
    }.unwrap_or_else(|| print_usage_exit(&cmd_name));

    let file_content = fs::read_to_string(input_file)
        .expect("input file should exist and be text file");

    let net = Network::load(&file_content);

    let count = if run_part2 {
        count_steps_multi_start(&net)
    } else {
        net.count_steps("AAA", "ZZZ")
    };

    println!("{count}");
}

// Sadly I failed to figure out this on my own.
// I cheated and looked up how other people solved it.
fn count_steps_multi_start(net: &Network) -> usize {
    let start_steps: Vec<_> = net.nodes.keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| k.as_str())
        .collect();

    start_steps.iter()
        .map(|start| net.count_steps(start, "Z"))
        .reduce(|a, b| a.lcm(&b))
        .unwrap()
}

fn print_usage_exit(me: &str) -> ! {
    println!("{me} input.txt");
    process::exit(1)
}

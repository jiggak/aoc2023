use std::{
    {env, fs, process},
    collections::HashMap
};

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
        let instructions: Vec<Instruction> = inst.chars()
            .map(|c| c.into())
            .collect();

        let mut nodes: HashMap<String, (String, String)> = HashMap::new();

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

        for inst in &self.instructions {
            let node = self.nodes.get(next).unwrap();
            count += 1;

            next = match inst {
                Instruction::Left => node.0.as_str(),
                Instruction::Right => node.1.as_str()
            };

            if next == end {
                return count;
            }
        }

        count + self.count_steps(next, end)
    }
}

fn main() {
    let mut args = env::args();

    // first arg is command name
    let cmd_name = args.next().unwrap();

    let input_file = match args.next() {
        Some(f) => f,
        None => {
            print_usage_exit(&cmd_name);
        }
    };

    let file_content = fs::read_to_string(input_file)
        .expect("input file should exist and be text file");

    let net = Network::load(&file_content);
    println!("{}", net.count_steps("AAA", "ZZZ"));
}

fn print_usage_exit(me: &str) -> ! {
    println!("{me} input.txt");
    process::exit(1)
}

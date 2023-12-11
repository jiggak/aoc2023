use std::{env, fs, process};

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
    }.unwrap_or_else(|| {
        println!("{cmd_name} input.txt");
        process::exit(1)
    });

    let file_content = fs::read_to_string(input_file)
        .expect("input file should exist and be text file");

    let total = if run_part2 {
        part2(&file_content)
    } else {
        part1(&file_content)
    };

    println!("{total}");
}

fn part1(data: &str) -> i32 {
    let mut total = 0;

    for line in data.lines() {
        let history: Vec<_> = line.split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let diffs = generate_diffs(history);

        let next_value: i32 = diffs.iter()
            .map(|s| s.last().unwrap())
            .rev()
            .scan(0, |acc, x| {
                *acc = *acc + *x;
                Some(*acc)
            })
            .last().unwrap();

        total += next_value;
    }

    total
}

fn part2(data: &str) -> i32 {
    let mut total = 0;

    for line in data.lines() {
        let history: Vec<_> = line.split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let diffs = generate_diffs(history);

        let first_value: i32 = diffs.iter()
            .map(|s| s.first().unwrap())
            .rev()
            .scan(0, |acc, x| {
                *acc = *x - *acc;
                Some(*acc)
            })
            .last().unwrap();

        total += first_value;
    }

    total
}

fn generate_diffs(history: Vec<i32>) -> Vec<Vec<i32>> {
    let mut diffs: Vec<Vec<i32>> = vec![history];

    loop {
        if let Some(history) = diff_pairs(&diffs.last().unwrap()) {
            diffs.push(history);
        } else {
            break;
        }
    }

    diffs
}

fn diff_pairs(history: &Vec<i32>) -> Option<Vec<i32>> {
    if history.iter().all(|n| *n == 0) {
        None
    } else {
        Some(history.iter()
            .zip(history.iter().skip(1))
            .map(|(a, b)| b-a)
            .collect())
    }
}

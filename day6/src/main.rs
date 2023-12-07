use std::{env, process, fs::read_to_string};

#[derive(Debug)]
struct RaceResult {
    duration: u64,
    distance: u64
}

impl RaceResult {
    fn new(race_duration: u64, distance: u64) -> Self {
        RaceResult { duration: race_duration, distance }
    }

    fn from_button_press(race_duration: u64, button_duration: u64) -> Self {
        // boat speed is X mm/s where X = button duration
        let remaining_duration = race_duration - button_duration;
        let mm_sec = button_duration;

        RaceResult {
            duration: race_duration,
            distance: remaining_duration * mm_sec
        }
    }

    fn count_winnable_button_durations(&self) -> usize {
        (1..self.duration).into_iter()
            .map(|d| RaceResult::from_button_press(self.duration, d))
            .filter(|r| r.distance > self.distance)
            .count()
    }

    // fn get_button_press_results(&self) -> impl Iterator<Item = Self>
    // {
    //     (1..self.duration).into_iter()
    //         .map(|d| RaceResult::from_button_press(self.duration, d))
    // }
}

fn load_race_results(data: &str) -> Vec<RaceResult> {
    let mut times: Vec<u64> = vec![];
    let mut distances: Vec<u64> = vec![];

    for line in data.lines() {
        if line.starts_with("Time:") {
            times = line.replace("Time:", "")
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
        }
        if line.starts_with("Distance:") {
            distances = line.replace("Distance:", "")
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
        }
    }

    times.into_iter()
        .zip(distances)
        .map(|(time, distance)| RaceResult::new(time, distance))
        .collect()
}

fn load_single_race_result(data: &str) -> RaceResult {
    let mut time: u64 = 0;
    let mut distance: u64 = 0;

    for line in data.lines() {
        if line.starts_with("Time:") {
            time = line.replace("Time:", "")
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect::<String>()
                .parse::<u64>().unwrap();
        }
        if line.starts_with("Distance:") {
            distance = line.replace("Distance:", "")
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect::<String>()
                .parse::<u64>().unwrap();
        }
    }

    RaceResult::new(time, distance)
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

    let file_content = read_to_string(input_file)
        .expect("input file should exist and be text file");

    let total = if run_part2 {
        part2(&file_content)
    } else {
        part1(&file_content)
    };

    println!("{total}");
}

fn print_usage_exit(me: &str) -> ! {
    println!("{me} [-p2] input.txt");
    process::exit(1)
}

fn part1(file_content: &str) -> usize {
    let mut total = 1usize;
    let results = load_race_results(file_content);

    for race in results {
        total *= race.count_winnable_button_durations();
    }

    total
}

fn part2(file_content: &str) -> usize {
    let result = load_single_race_result(file_content);
    result.count_winnable_button_durations()
}

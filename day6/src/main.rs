use std::{env, process, fs::read_to_string};

#[derive(Debug)]
struct RaceResult {
    duration: u32,
    distance: u32
}

impl RaceResult {
    fn new(race_duration: u32, distance: u32) -> Self {
        RaceResult { duration: race_duration, distance }
    }

    fn from_button_press(race_duration: u32, button_duration: u32) -> Self {
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
    let mut times: Vec<u32> = vec![];
    let mut distances: Vec<u32> = vec![];

    for line in data.lines() {
        if line.starts_with("Time:") {
            times = line.replace("Time:", "")
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
        }
        if line.starts_with("Distance:") {
            distances = line.replace("Distance:", "")
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
        }
    }

    return times.into_iter()
        .zip(distances)
        .map(|(time, distance)| RaceResult::new(time, distance))
        .collect();
}

fn main() {
    let mut args = env::args();

    // first arg is command name
    let cmd_name = args.next().unwrap();

    let input_file = match args.next() {
        Some(f) => f,
        None => {
            println!("{cmd_name} [input.txt]");
            process::exit(1);
        }
    };

    let file_content = read_to_string(input_file)
        .expect("input file should exist and be text file");

    let mut total = 1;
    let results = load_race_results(file_content.as_str());

    for race in results {
        total *= race.count_winnable_button_durations();
    }

    println!("{total}");
}

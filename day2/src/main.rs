use std::{cmp, collections::HashMap, env, fs::read_to_string, process, str::FromStr};

#[derive(Debug, Hash, PartialEq, Eq)]
enum CubeColor {
    Red,
    Green,
    Blue
}

impl FromStr for CubeColor {
    type Err = ();

    fn from_str(input: &str) -> Result<CubeColor, Self::Err> {
        match input {
            "red" => Ok(CubeColor::Red),
            "green" => Ok(CubeColor::Green),
            "blue" => Ok(CubeColor::Blue),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
struct Game {
    number: i32,
    results: Vec<HashMap<CubeColor, i32>>
}

impl Game {
    fn is_possible(&self, rgb: (i32, i32, i32)) -> bool {
        self.results.iter().all(|set| {
            set.get(&CubeColor::Red).map_or(true, |v| *v <= rgb.0) &&
            set.get(&CubeColor::Green).map_or(true, |v| *v <= rgb.1) &&
            set.get(&CubeColor::Blue).map_or(true, |v| *v <= rgb.2)
        })
    }

    fn max_cube_set(&self) -> (i32, i32, i32) {
        let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);

        for set in &self.results {
            if let Some(red) = set.get(&CubeColor::Red) {
                max_red = cmp::max(max_red, *red);
            }

            if let Some(green) = set.get(&CubeColor::Green) {
                max_green = cmp::max(max_green, *green);
            }

            if let Some(blue) = set.get(&CubeColor::Blue) {
                max_blue = cmp::max(max_blue, *blue);
            }
        }

        (max_red, max_green, max_blue)
    }
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

    //
    let part2 = env::var("PART2").is_ok();

    let mut total = 0;

    for line in read_to_string(input_file).expect("input").lines() {
        let mut line_parts = line.splitn(2, ':');
        let (mut game, cube_sets) = (
            line_parts.next().expect("game prefix").splitn(2, ' '),
            line_parts.next().expect("cube sets").trim().split(';')
        );

        game.next().expect("literal 'Game'");
        let game_number = game.next()
            .expect("game number")
            .parse::<i32>()
            .expect("game number integer");

        let game = Game {
            number: game_number,
            results: cube_sets.map(|set| {
                set.trim().split(',').map(|x| {
                    let mut cube_parts = x.trim().splitn(2, ' ');
                    let (count, color) = (
                        cube_parts.next().expect("cube count").parse::<i32>().expect("cube count integer"),
                        CubeColor::from_str(cube_parts.next().expect("cube color")).expect("cube color")
                    );
                    (color, count)
                }).collect()
            }).collect()
        };

        if part2 {
            let max_set = game.max_cube_set();
            total += max_set.0 * max_set.1 * max_set.2;
        } else {
            if game.is_possible((12, 13, 14)) {
                total += game.number;
            }
        }
    }

    println!("{total}");
}

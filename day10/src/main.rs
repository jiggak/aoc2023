use std::{env, fs, process};

struct PipeArea {
    pipes: Vec<PipeTile>,
    cols: usize,
    rows: usize
}

impl PipeArea {
    fn load(data: &str) -> Self {
        let mut pipes = vec![];
        let mut cols = None;

        for line in data.lines() {
            if cols.is_none() {
                cols = Some(line.len());
            }

            pipes.extend(line.chars().map(|c| PipeTile::from(c)));
        }

        let rows = pipes.len() / cols.unwrap();

        Self {
            pipes,
            cols: cols.unwrap(),
            rows
        }
    }

    fn count_loop_steps(&self) -> usize {
        let start_index = self.pipes.iter()
            .position(|pipe| *pipe == PipeTile::Start)
            .expect("Pipes should have start tile");

        // get first direction that has pipe tile next to start
        let start_direction = Cardinal::iter()
            .find(|dir| self.has_connection(start_index, dir))
            .unwrap();

        let mut next = (start_index, Some(*start_direction));
        let mut count = 0;

        // println!("start {} {:?}", next.0, next.1);

        loop {
            next = self.next(next.0, &next.1.unwrap());
            // println!("next {} {:?}", next.0, next.1);
            if self.pipes[next.0] == PipeTile::Start {
                break;
            }

            count += 1;
        }

        count
    }

    fn index_to_point(&self, index: usize) -> (i32, i32) {
        (
            (index % self.cols) as i32,
            (index / self.cols) as i32
        )
    }

    fn point_to_index(&self, point: (i32, i32)) -> usize {
        point.1 as usize * self.cols + point.0 as usize
    }

    fn next(&self, from_index: usize, dir: &Cardinal) -> (usize, Option<Cardinal>) {
        let next_index = self.next_index(from_index, dir).unwrap();

        if let Some((dir1, dir2)) = self.pipes[next_index].directions() {
            if dir.opposite() == dir1 {
                (next_index, Some(dir2))
            } else {
                (next_index, Some(dir1))
            }
        } else {
            (next_index, None)
        }
    }

    /// Returns true if there is a pipe connection from the index in the given direction
    fn has_connection(&self, from_index: usize, dir: &Cardinal) -> bool {
        self.next_index(from_index, dir)
            .map(|i| self.pipes[i].has_direction(&dir.opposite()))
            .unwrap_or_default()
    }

    fn next_index(&self, from_index: usize, dir: &Cardinal) -> Option<usize> {
        let limit_x = (self.cols - 1) as i32;
        let limit_y = (self.rows - 1) as i32;

        let (mut point_x, mut point_y) = self.index_to_point(from_index);

        match dir {
            Cardinal::North => point_y -= 1,
            Cardinal::East => point_x += 1,
            Cardinal::South => point_y += 1,
            Cardinal::West => point_x -= 1
        }

        if point_x < 0 || point_x > limit_x || point_y < 0 || point_y > limit_y {
            None
        } else {
            Some(self.point_to_index((point_x, point_y)))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cardinal {
    North,
    East,
    South,
    West
}

impl Cardinal {
    fn iter() -> impl Iterator<Item = &'static Cardinal> {
        [Cardinal::North, Cardinal::East, Cardinal::South, Cardinal::West].iter()
    }

    fn opposite(&self) -> Self {
        match self {
            Cardinal::North => Cardinal::South,
            Cardinal::East => Cardinal::West,
            Cardinal::South => Cardinal::North,
            Cardinal::West => Cardinal::East
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PipeTile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start
}

impl PipeTile {
    fn from(c: char) -> Self {
        match c {
            '|' => PipeTile::NorthSouth,
            '-' => PipeTile::EastWest,
            'L' => PipeTile::NorthEast,
            'J' => PipeTile::NorthWest,
            '7' => PipeTile::SouthWest,
            'F' => PipeTile::SouthEast,
            '.' => PipeTile::Ground,
            'S' => PipeTile::Start,
            _ => panic!("Unexpected char {c}")
        }
    }

    fn directions(&self) -> Option<(Cardinal, Cardinal)> {
        match self {
            PipeTile::NorthSouth => Some((Cardinal::North, Cardinal::South)),
            PipeTile::EastWest => Some((Cardinal::East, Cardinal::West)),
            PipeTile::NorthEast => Some((Cardinal::North, Cardinal::East)),
            PipeTile::NorthWest => Some((Cardinal::North, Cardinal::West)),
            PipeTile::SouthWest => Some((Cardinal::South, Cardinal::West)),
            PipeTile::SouthEast => Some((Cardinal::South, Cardinal::East)),
            _ => None
        }
    }

    fn has_direction(&self, d: &Cardinal) -> bool {
        if let Some((dir1, dir2)) = self.directions() {
            dir1 == *d || dir2 == *d
        } else {
            false
        }
    }
}

fn main() {
    let mut args = env::args();

    let cmd_name = args.next().unwrap();

    let mut _run_part2 = false;
    let input_file = match args.next() {
        Some(a) if a == "-p2" => {
            _run_part2 = true;
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

    let area = PipeArea::load(&file_content);
    println!("{:?}", (area.count_loop_steps() + 1) / 2);
}

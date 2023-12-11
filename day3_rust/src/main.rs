use std::{collections::HashSet, env, fs::read_to_string, process};

enum Cell {
    Space,
    Number { content: char },
    Symbol { content: char }
}

struct Board {
    cells: Vec<Cell>,
    cols: usize,
    rows: usize
}

struct Point {
    x: i32,
    y: i32
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        if c == '.' {
            Cell::Space
        } else if c.is_digit(10) {
            Cell::Number { content: c }
        } else {
            Cell::Symbol { content: c }
        }
    }
}

impl Board {
    fn load(data: String) -> Self {
        let mut rows: usize = 0;
        let mut cols: Option<usize> = None;
        let mut cells: Vec<Cell> = vec![];

        for line in data.lines() {
            if cols.is_none() {
                cols = Some(line.len());
            }

            for c in line.chars() {
                cells.push(Cell::from(c));
            }

            rows += 1;
        }

        Board {
            cols: cols.unwrap(),
            rows,
            cells
        }
    }

    fn part_number_total(&self) -> u32 {
        let mut total = 0;

        for (i, cell) in self.cells.iter().enumerate() {
            if let Cell::Symbol { .. } = cell {
                let numbers = self.adjacent_numbers(i);
                total += numbers.iter().sum::<u32>();
            }
        }

        total
    }

    fn gear_ratio_total(&self) -> u32 {
        let mut total = 0;

        for (i, cell) in self.cells.iter().enumerate() {
            if let Cell::Symbol { content: '*' } = cell {
                let numbers = self.adjacent_numbers(i);
                if numbers.len() == 2 {
                    total += numbers.iter().product::<u32>();
                }
            }
        }

        total
    }

    // FIXME this doesn't account for numbers that are adjacent to multiple
    // symbols, it will include those numbers twice in the results.
    // This doesn't seem to matter with the puzzle input data, but it bugs me.
    fn adjacent_numbers(&self, index: usize) -> HashSet<u32> {
        let point = self.index_to_point(index);

        let limit_x = (self.cols - 1) as i32;
        let limit_y = (self.rows - 1) as i32;

        let mut search: Vec<Point> = vec![];

        let x_range = (point.x - 1) .. (point.x + 2);

        // generate search point rect around symbol from top left
        search.extend(x_range.clone().map(|x| Point { x, y: point.y - 1 }));
        search.push(Point { x: point.x + 1, y: point.y });
        search.extend(x_range.map(|x| Point { x, y: point.y + 1 }));
        search.push(Point { x: point.x - 1, y: point.y });

        let mut result: HashSet<u32> = HashSet::new();

        for p in search {
            if p.x < 0 || p.x > limit_x || point.y < 0 || point.y > limit_y {
                // skip if point out of range
                continue;
            }

            let i = self.point_to_index(&p);
            let val = self.number_at_index(i);
            if let Some(val) = val {
                result.insert(val);
            }
        }

        result
    }

    fn index_to_point(&self, index: usize) -> Point {
        Point {
            x: (index % self.cols) as i32,
            y: (index / self.cols) as i32
        }
    }

    fn point_to_index(&self, point: &Point) -> usize {
        point.y as usize * self.cols + point.x as usize
    }

    fn number_at_index(&self, index: usize) -> Option<u32> {
        if let Cell::Number { .. } = &self.cells[index] {
            let mut iter = self.cells[..=index].iter()
                .enumerate()
                .rev();

            // iter reverse until cell is not number and start one forward
            let start = iter.find_map(|(i, cell)| match cell {
                Cell::Number { .. } => None,
                _ => Some(i)
            }).map_or(0, |i| i + 1);

            // now collect/concat all number cells and parse into int
            let number = self.cells[start..].iter()
                .map_while(|cell| match cell {
                    Cell::Number { content: c } => Some(c),
                    _ => None
                }).collect::<String>();

            Some(number.parse::<u32>().expect("number should parse as u32"))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_at_index() {
        let board = Board::load("467..114..".into());

        assert_eq!(board.number_at_index(1), Some(467));
        assert_eq!(board.number_at_index(4), None);
        assert_eq!(board.number_at_index(6), Some(114));
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
    }.unwrap_or_else(|| {
        println!("{cmd_name} [input.txt]");
        process::exit(1);
    });

    let file_content = read_to_string(input_file)
        .expect("input file should exist and be text file");

    let board = Board::load(file_content);

    let total = if !run_part2 {
        board.part_number_total()
    } else {
        board.gear_ratio_total()
    };

    println!("{total}");
}
